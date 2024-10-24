module.exports = {
  overrides: [
    {
      files: ["./graphql/*.graphql"],
      extends: [
        "plugin:@graphql-eslint/operations-recommended",
        "plugin:@graphql-eslint/schema-recommended",
      ],
      parser: "@graphql-eslint/eslint-plugin",
      plugins: ["@graphql-eslint"],
      parserOptions: {
        schema: "./graphql/schema.json",
        operations: ["./graphql/*.graphql"],
      },
    },
  ],
};
