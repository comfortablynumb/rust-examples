# GraphQL Server

Modern API query language allowing clients to request exactly the data they need.

## Concepts Covered

- GraphQL schema definition
- Queries and resolvers
- Type system
- Nested queries
- Field selection
- Arguments and filtering

## GraphQL vs REST

| Feature | GraphQL | REST |
|---------|---------|------|
| Data Fetching | Request specific fields | Fixed endpoints |
| Over-fetching | No | Common |
| Under-fetching | No | Common |
| Versioning | Not needed | Required |
| Endpoints | Single | Multiple |
| Type System | Strong | Weak |

## Schema Definition

```rust
#[derive(SimpleObject)]
struct Book {
    id: i32,
    title: String,
    author: String,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn book(&self, id: i32) -> Option<Book> {
        // Resolver logic
    }

    async fn books(&self) -> Vec<Book> {
        // Return all books
    }
}
```

## Query Examples

### Get All Books

```graphql
query {
  books {
    id
    title
    author
    year
  }
}
```

### Get Specific Fields

```graphql
query {
  books {
    title
    author
  }
}
```

### Query with Parameters

```graphql
query {
  book(id: 1) {
    title
    author
  }
}
```

### Nested Queries

```graphql
query {
  author(id: 1) {
    name
    books {
      title
      year
    }
  }
}
```

### Search

```graphql
query {
  searchBooks(query: "rust") {
    title
    author
  }
}
```

## Running

```bash
cargo run
```

Then:

```bash
# Query with curl
curl -X POST http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query": "{ books { title } }"}'

# Or open http://localhost:8000 in browser
```

## Advantages

1. **No Over-fetching**: Get exactly what you need
2. **No Under-fetching**: One request for multiple resources
3. **Strong Typing**: Schema validates queries
4. **Introspection**: API self-documenting
5. **Rapid Development**: Frontend-driven API evolution

## Schema Features

### Types

```graphql
type Book {
  id: Int!
  title: String!
  author: String!
  year: Int
}
```

### Queries

```graphql
type Query {
  book(id: Int!): Book
  books: [Book!]!
  searchBooks(query: String!): [Book!]!
}
```

### Mutations

```graphql
type Mutation {
  addBook(title: String!, author: String!): Book!
  deleteBook(id: Int!): Boolean!
}
```

### Subscriptions

```graphql
type Subscription {
  bookAdded: Book!
}
```

## Best Practices

1. **Design Schema Carefully**: Think about data relationships
2. **Use DataLoader**: Batch and cache database queries
3. **Implement Pagination**: Limit large result sets
4. **Error Handling**: Return meaningful errors
5. **Authentication**: Protect sensitive queries
6. **Monitoring**: Track query performance

## Common Patterns

### Pagination

```graphql
query {
  books(first: 10, after: "cursor") {
    edges {
      node {
        title
      }
      cursor
    }
    pageInfo {
      hasNextPage
    }
  }
}
```

### Filtering

```graphql
query {
  books(filter: { year: 2021, author: "Alice" }) {
    title
  }
}
```

### Aliases

```graphql
query {
  rust: searchBooks(query: "rust") { title }
  python: searchBooks(query: "python") { title }
}
```

## Use Cases

- Mobile apps (bandwidth efficiency)
- Microservices aggregation
- Real-time applications
- Content management systems
- Dashboard applications
- API gateways

## Tools

- **GraphQL Playground**: Interactive query IDE
- **GraphiQL**: In-browser IDE
- **Apollo Client**: Frontend integration
- **DataLoader**: Batching and caching

## References

- [GraphQL Specification](https://spec.graphql.org/)
- [async-graphql Documentation](https://async-graphql.github.io/async-graphql/)
- [GraphQL Best Practices](https://graphql.org/learn/best-practices/)
