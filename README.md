# Company XYZ Backend coding challenge

## Context

Company XYZ provides e-commerce shops with a flexible payment method that allows shoppers to split their purchases in three months without any cost. In exchange, Company XYZ earns a fee for each purchase.

When shoppers use this payment method, they pay directly to Company XYZ. Then, Company XYZ disburses the orders to merchants with different frequencies and pricing.

This challenge is about implementing the process of paying merchants.

## Problem statement

We have to implement a system to automate the calculation of merchants’ disbursements payouts and Company XYZ commissions for existing, present in the CSV files, and new orders.

The system must comply with the following requirements:

- All orders must be disbursed precisely once.
- Each disbursement, the group of orders paid on the same date for a merchant, must have a unique alphanumerical `reference`.
- Orders, amounts, and fees included in disbursements must be easily identifiable for reporting purposes.

The disbursements calculation process must be completed, for all merchants, by 8:00 UTC daily, only including those merchants that fulfill the requirements to be disbursed on that day. Merchants can be disbursed daily or weekly. We will make weekly disbursements on the same weekday as their `live_on` date (when the merchant started using Company XYZ, present in the CSV files). Disbursements groups all the orders for a merchant in a given day or week.

For each order included in a disbursement, Company XYZ will take a commission, which will be subtracted from the merchant order value gross of the current disbursement, following this pricing:

- `1.00 % fee` for orders with an amount strictly smaller than `50 €`.
- `0.95 % fee` for orders with an amount between 50 € and `300 €`.
- `0.85 % fee` for orders with an amount of `300 €` or more.

**Remember that we are dealing with money, so we should be careful with related operations. In this case, we should round up to two decimals following**

Lastly, on the first disbursement of each month, we have to ensure the `minimum_monthly_fee` for the previous month was reached. The `minimum_monthly_fee` ensures that Company XYZ earns at least a given amount for each merchant.

When a merchant generates less than the `minimum_monthly_fee` of orders’ commissions in the previous month, we will charge the amount left, up to the `minimum_monthly_fee` configured, as "monthly fee". Nothing will be charged if the merchant generated more fees than the `minimum_monthly_fee`.

Charging the `minimum_monthly_fee` is out of the scope of this challenge. It is not subtracted from the disbursement commissions. Just calculate and store it for later usage.

## DATA

### Merchants sample

```
id                                   | REFERENCE                 | EMAIL                             | LIVE_ON    | DISBURSEMENT_FREQUENCY | MINIMUM_MONTHLY_FEE
2ae89f6d-e210-4993-b4d1-0bd2d279da62 | treutel_schumm_fadel      | info@treutel-schumm-and-fadel.com | 2022-01-01 | WEEKLY                 | 29.0
6596b87d-7f13-460f-ba1a-00872c770092 | windler_and_sons          | info@windler-and-sons.com         | 2021-05-25 | DAILY                  | 29.0
70de4478-bfa8-4c4c-97f1-4a0a149f8264 | mraz_and_sons             | info@mraz-and-sons.com            | 2020-03-20 | WEEKLY                 |  0.0
52f0e308-4a9d-4b32-ace4-c491f457d9a5 | cummerata_llc             | info@cummerata-llc.com            | 2019-02-04 | DAILY                  | 35.0
```

### Orders samples

```
id           | MERCHANT REFERENCE      | AMOUNT | CREATED AT
056d024481a9 | treutel_schumm_fadel    |  61.74 | 2023-01-01
33c80364591c | cummerata_llc           | 293.08 | 2023-01-01
5eaeabf54862 | mraz_and_sons           | 373.33 | 2023-01-01
70530cdc7b59 | treutel_schumm_fadel    |  60.48 | 2023-01-01
871e0d072782 | mraz_and_sons           | 213.97 | 2023-01-01
```

We expect you to:

- Create the necessary data structures and a way to persist them for the provided data. You don’t have to follow CSV’s schema if you think another one suits you better.
- Calculate and store the disbursements following described requirements for all the orders included in the CSV, and prepare the system to do the same for new orders.
- Fill the following table and include it in the README.

| Year | Number of disbursements | Amount disbursed to merchants | Amount of order fees | Number of monthly fees charged (From minimum monthly fee) | Amount of monthly fee charged (From minimum monthly fee) |
| --- | --- | --- | --- | --- | --- |
| 2022 | 365 | 50.023,45 € | 750,34 € | 50  | 650,00 € |
| 2023 | 400 | 75.000,35 € | 950,43 € | 49  | 750,00 € |

## Instructions

Please read carefully the challenge, and if you have any doubts or need extra info, please don’t hesitate to ask us before starting.

*   Create a **README** explaining:
    1.  How to **set up and and run** your solution
    2.  An explanation of your technical choices, tradeoffs, assumptions you took, etc.
    3.  If you left things aside due to time constraints, explain why and how you would resolve or improve them.
*   You should **consider this code ready for production** as it was a PR to be reviewed by a colleague. Also, commit as if it were a real-world feature.
*   **Design, test, develop and document the code.** It should be a performant, clean, and well-structured solution.
*   You **shouldn’t spend more than 6h** on the challenge.
*   You can code the solution in a language of your choice. Here are some technologies we are more familiar with (in no particular order): JavaScript, Ruby, Python, Go, Elixir, Java, Kotlin, and PHP.
*   Your **experience level will be taken into consideration** when evaluating.

# Solution/Approach
**Event driven architecture approach**

One component called "calculator" calculates the data/results and publishes an event to confluent/kafka with this info.
The "results" component consumes this event and update the table with these results.

## Context
Because we probably have specific scalability needs, we'll opt for an event-driven architecture approach in this exercise. If there are no deadlocks and the database isn't a bottleneck, either because it's a small app or there's no need to scale, we could build a monolithic architecture with a single database and using joins with an anti-corruption layer, moving the logic to the infrastructure.

Once per day at 00:01 utc, we run a cron job to import the new merchants and orders from the csv. It would be nice to receive an event directly in the calculator context but we need to create an importer since is how we receive these data.

This is the flow (it's work in progress) for this app:

```mermaid
sequenceDiagram
  participant ImporterDB as Importer DB
  participant Importer
  participant Kafka
  participant CalculatorDB as Calculator DB
  participant Calculator
  participant MetricsDB as Metrics DB
  participant Metrics

  %% Import merchants from CSV with upsert
  Importer->>ImporterDB: Upsert Merchants (insert/update)
  Note right of ImporterDB: Importer DB stores the canonical merchant data
  Importer->>Kafka: merchant_upserted
  Kafka-->>Calculator: merchant_upserted
  Calculator->>CalculatorDB: Project Merchant State
  Note right of CalculatorDB: Calculator DB stores latest merchant projection

  %% New orders arrive
  Importer->>Kafka: order_created
  Kafka-->>Calculator: order_created
  Calculator->>CalculatorDB: Save Order (linked to Merchant)

  %% Daily disbursement process (00:01 to 08:00 UTC)
  Calculator->>CalculatorDB: Process Disbursements
  Calculator->>Kafka: disbursement_created
  Calculator->>Kafka: metrics_calculated

  %% Metrics consume the already calculated projections
  Kafka-->>Metrics: metrics_calculated
  Metrics->>MetricsDB: Store Yearly Aggregates
```

# State of this project

WIP (Work in progress)

# Payouts Calculator - Importer & Calculator Services

This project handles the import of merchants from CSV files, processes orders, calculates disbursements, and publishes projections to Kafka.

## Prerequisites

- Rust toolchain installed (`rustc`, `cargo`)
- Docker & Docker Compose installed
- PostgreSQL and Kafka run via Docker Compose

## Starting the Environment

Start all services with Docker Compose:

`docker-compose up -d`

Check the containers are running:

`docker ps`

## Database Setup
Apply the migrations for development and test databases:

```
# you must run this sqitch commands since the importer/calculator migrations folder:

# For importer development database.
sqitch deploy db:pg://postgres:postgres@localhost:5432/importer_db


# For importer test database
sqitch deploy db:pg://postgres:postgres@localhost:5432/importer_test_db

# For calculator development database.
sqitch deploy db:pg://postgres:postgres@localhost:5433/calculator_db

# For calculator test database.
sqitch deploy db:pg://postgres:postgres@localhost:5433/calculator_test_db
```

### Importer Database

The Importer crate uses its own database. By default:

- Development DB: `importer_db`
- Test DB: `importer_test_db`

Connect to the development database:

`docker exec -it postgres-local psql -U postgres -d importer_db`

To see the `merchants` table:

`SELECT * FROM merchants;`

To clear the table:

`TRUNCATE TABLE merchants;`


### Calculator Database

The Calculator crate uses its own database. By default:

- Development DB: `calculator_db`
- Test DB: `calculator_test_db`

Connect to the development database:

`docker exec -it calculator_db psql -U postgres -d calculator_db`

To see the `merchants` table:

`SELECT * FROM merchants;`

To clear the table:

`TRUNCATE TABLE merchants;`

## Environment Variables

The project loads configuration from `.env` or `.env.test`. Key variables:

- `APP_ENV` – `development` or `test`
- `CSV_PATH` – path to the CSV file
- `DATABASE_URL` – PostgreSQL connection URL
- `KAFKA_BROKERS` – Kafka broker list

Example `.env`:

```
APP_ENV=development
CSV_PATH=cron_runner/data/merchants.csv
CALCULATOR_DATABASE_URL=postgres://postgres:postgres@localhost:5433/calculator_db
IMPORTER_DATABASE_URL=postgres://postgres:postgres@localhost:5432/importer_db
KAFKA_BROKERS=localhost:9092
```

Example `.env.test`:

```
APP_ENV=test
CSV_PATH=tests/fixtures/merchants.csv
IMPORTER_DATABASE_URL=postgres://postgres:postgres@localhost:5432/importer_test_db
CALCULATOR_DATABASE_URL=postgres://postgres:postgres@localhost:5432/calculator_test_db
KAFKA_BROKERS=localhost:9092
```

## Running the Importer

To run the importer in development:

`cargo run -p importer --bin cron_runner`

This will:

- Read merchants from the CSV
- Upsert them into the database
- Publish `merchant_upserted` events to Kafka

## Running the Calculator

To run the importer in development:

`cargo run -p calculator --bin calculator_consumer`

This will:

- Consume the merchants from the importer
- Upsert merchants to the calculators db

## Running Tests

To run all tests for the importer crate:

`cargo test -p importer -- --nocapture --test-threads=1`

This ensures:
```
- The database is cleaned before each test
- Merchants are imported and upserted
- Events are published and verified via mocks
```

## Verifying Data in Database

Connect to the test database:

`docker exec -it postgres-local psql -U postgres -d importer_test_db`

Check merchants:

`SELECT * FROM merchants;`

## Reading Kafka Projections

You can use Kafka CLI tools to inspect the `merchant_upserted` topic. For example:

`kafkacat -b localhost:9092 -t merchant_upserted -C -K: -f '\nKey: %k\nPayload: %s\n--\n'`

This will show the latest state of merchants after upserts.

## Notes

- The importer now uses **upserts** for efficiency. Existing merchants are updated, new ones inserted.
- The calculator consumes and upserts the merchants to the calculator db.
- Kafka events reflect the **latest merchant state** (projections), not individual insert events.
- Tests use `.env.test` and a dedicated test database.
- Make sure Docker containers are running before running tests or the importer with `docker-compose up -d`
