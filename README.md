<img width="100%" src="./assets/banner.svg">

<div align="right">
  <a href="https://notbyai.fyi/">
  <img src="./assets/not-by-ai.png">
  </a>
</div>

---

## 🚧 CAUTION

- This project is in such early stages that it doesn't even qualify to be a member of reality.
- It's always great to work with source control, and I might as well make it public for people to gander if they so please.
- In the case that the project takes off, I will likely end up publishing it on [`crates.io`](https://crates.io/).

## ✨ Focus and Goals

`rs->sql`(or `rs2sql`) is an ambitious project to _rustify the SQL language_.

The project stemmed from my use of SQL within past projects, where I asked myself "how would Rust write this query?".
The idea is to create a type system + functionality to recreate SQL queries in functional Rust code, and provide a translation into SQL.

As a first draft I have something like this in mind:
```rust
let query = {
    let monthly_cost = {
        // the schema is used to check for correctness in identifiers and operatations
        let schema = Schema::from_file("schema.sql").unwrap(); 
        let monthly_cost = schema
            .table["Marketing"]
            .filter(|t| {
                let time_now = now();
                time_now - Interval::new(3, TimeUnit::Month) < created_date && created_date < time_now    
            })
            .order_by[[1, 2]]
            .group_by[[1, 2]]
            .select(|view| {
                let month = to_char(created_date, "YYYY-MM");
                vec![
                    view["campaign_id"].as("campaign"),
                    month,
                    sum(view["cost"]).as("monthly_cost")
                ]
            })
            .as("Cost_By_Month")
    };

    monthly_cost
        .group_by("campaign")
        .order_by("campaign")
        .select(|v| {
            vec![v["campaign"], avg(v["monthly_cost"]).as("Avg Monthly Cost")]
        })
        .to_sql()
});

println!("{}", query);
```
outputting
```sql
SELECT campaign, avg(monthly_cost) as "Avg Monthly Cost"
FROM
    (SELECT campaign_id AS campaign,
       TO_CHAR(created_date, 'YYYY-MM') AS month,
       SUM(cost) AS monthly_cost
    FROM marketing
    WHERE created_date BETWEEN NOW() - INTERVAL '3 MONTH' AND NOW()
    GROUP BY 1, 2
    ORDER BY 1, 2) as Cost_By_Month
GROUP BY campaign
ORDER BY campaign
```

- It would also be great to have a derived-schema implementation. This implementation doesn't take an initial schema which it uses to ensure all the identifiers and operations make sense, but instead starts with a clean slate and tries to recreate the schema from the uses. It should infer the types and properties of objects and store the information, such that any contradicting information results in some failure:
```rust
let schema = Schema::derive(); // schema is built along with the operations that use it
// all of the following accesses are added into the schema. any clashes would result in an error
let view1 = schema.table["post_likes"].select(|t| vec![t["post_id"], t["likes"] + 1]) // `likes` is now enforced to be numeric
let view2 = view.select(|v| vec![v["post_id"], v["likes"] + "text"]) // this would fail because you cant add a numeric and a string
```

## 📚 Alternative Ideas

### 1. Cursed Comptime `TokenStream`-Parsing Macros
- Forget everything about having a complete backing type system with implementations that help you write and check the queries in Rust.
- Simply focus on what the Rust grammar allows, and encode SQL queries into the Rust syntax.
- Delegate all of the translation into the evaluation of code blocks using a comptime macros that parse the `TokenStream`s.

NOTE: I don't actually know if it is possible to get the `TokenStream` of a block passed in to a function-like proc macro. 

```rust
let query = rsql!({
    "employee"
        .select( |employee_id, last_name, first_name, salary| {
                // `self` grabs the outer table/view (in this case "`employee`")
                let ranking = (self.order_by("salary", Desc)).rank();
                (employee_id, last_name, first_name, salary, ranking)
        })
        .order_by("ranking", Asc)
        .limit(5)
})

println!("{}", query);
```
outputting
```
SELECT
    employee_id,
    last_name,
    first_name,
    salary,
    RANK() OVER (ORDER BY salary DESC) as ranking
FROM employee
ORDER BY ranking
LIMIT 5
```

### 2. `SQL++` or `CSQL` (Concise SQL) 
- If we consider using `TokenStream` macros with deep logic, why not just create our own new language that transpiles into SQL?
- As a bonus we can make good use of unicode characters--like popular array-based languages do--in order to condense the syntax heavily.
- The end goal of this would be to simply have a binary that transfers our new language files into SQL files.

```bash
$ query="[employee⊚[salary>5000]⊏[employee_id,last_name,first_name,salary,☇[⇌[↧,salary]]=ranking]]⇌[ranking]"
$ sql++ -i $query
SELECT 
    employee_id,
    last_name,
    first_name,
    salary,
    RANK() OVER (ORDER BY salary DESC) as ranking
FROM employee
WHERE salary > 50000
ORDER BY ranking
$
```

## 🤨 Why??

I greatly enjoy writing Rust! 
Projects like these always start out as a way to have fun and improve. They provide a challenging yet creative environment to practice and learn about Rust!
