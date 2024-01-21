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

## ✨ Project Focus and Goals

`rs->sql`(or `rs2sql`) is an ambitious project to _rustify the SQL language_.

The project stemmed from my use of SQL within past projects, where I asked myself "how would Rust write this query?".

As such, the project is split into two main parts:
1. Generate a backing type system and functionality to recreate SQL queries in "idiomatic" Rust.
2. Provide a macro-based parser in order to translate the "Rust query" into SQL.

SQL's grammar is difficult to transpile into Rust's grammar. This is one of the fun parts of the project where I try to figure out how best to accomplish this.
I hope to use a wide variety of custom types and functions along with a careful balance of `TokenStream`-parsing macros (used to cut down on the syntax).
The point is to have the freedom to rewrite SQL into my own thing, while leveraging Rust's grammar, LSP and compiler in order to enforce query correctness as much as possible.

As a first draft I have something like this in mind:
```rust
let query = rsql!({
    let monthly_cost = {
        let table = Table::name("Marketing");

        let filter = filter!(table, |created_date| {
            let time_now = now();
            time_now - Interval::new(3, TimeUnit::Month) < created_date && created_date < time_now    
        });

        let view = filter.order_by_num(&[1, 2])
                         .group_by_num(&[1, 2]);

        select!(view, |campaign_id, created_date, cost| {
            let month = to_char(created_date, "YYYY-MM");
            (
                campaign_id.as("campaign"),
                month,
                sum!(cost).as("monthly_cost")
            )
        }.as("Cost_By_Month")
    };

    let view = monthly_cost.group_by("campaign")
                           .order_by("campaign");

    select!(view, |campaign, monthly_cost| {
        (campaign, avg!(monthly_cost).as("Avg Monthly Cost"))
    }
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

## 🤨 Why??

There's a high likelihood that you think the Rust version of the query looks worse than the SQL version. I agree. 

With time I hope to find simpler implementations that translate the SQL syntax into Rust. 
For now I just want to have fun with the large creativity that implementing all of this entails, while getting better at writing Rust :)
