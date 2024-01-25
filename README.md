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
        let schema: CompiledSchema = Schema::from_file("schema.sql").unwrap(); 
        let monthly_cost = schema
            .table("Marketing")
            .filter(|t| {
                let created_date = t["created_date"];
                let time_now = now();
                time_now - Interval::new(3, TimeUnit::Month) < created_date && created_date < time_now    
            })
            .order_by([1, 2]) // generic functions that take care of casting
            .group_by([1, 2]) // 
            .select(|view| {
                let month = to_char(view["created_date"], "YYYY-MM");
                vec![
                    view["campaign_id"].as("campaign"),
                    month,
                    sum(view["cost"]).as("monthly_cost")
                ]
            })
            .as("Cost_By_Month")
    };

    monthly_cost
        .group_by("campaign") // generic functions that take care of casting
        .order_by("campaign") // 
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


## 🤨 Why??

I greatly enjoy writing Rust! 
Projects like these always start out as a way to have fun and improve. They provide a challenging yet creative environment to practice and learn about Rust!
