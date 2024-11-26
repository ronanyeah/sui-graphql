import graphqlPlugin from "@graphql-eslint/eslint-plugin";

export default [
  {
    files: ["graphql/*.graphql"],
    languageOptions: {
      parser: graphqlPlugin.parser,
      parserOptions: {
        graphQLConfig: {
          schema: "./graphql/schema.json",
          documents: "./graphql/*.graphql",
        },
      },
    },
    plugins: {
      "@graphql-eslint": graphqlPlugin,
    },
    rules: {
      ...graphqlPlugin.configs["flat/schema-recommended"].rules,
      ...graphqlPlugin.configs["flat/operations-recommended"].rules,
    },
  },
];
