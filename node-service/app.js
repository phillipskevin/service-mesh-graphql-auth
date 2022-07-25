const { ApolloServer, gql } = require('apollo-server');

const typeDefs = gql`
  type Person {
    name: String
    age: Int
    email: String
  }

  type Query {
    "A simple type for getting started!"
    people: [Person]
  }
`;

const resolvers = {
  Query: {
    people: () => [
      { name: "Connor", age: 6, email: "connor@people.com" },
      { name: "Dylan", age: 2, email: "dylan@people.com" }
    ]
  },
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

server.listen().then(({ url }) => {
  console.log(`🚀 Server ready at ${url}`);
});
