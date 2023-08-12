module.exports = {
    env: {
        browser: true,
        es2021: true
    },
    parser: "@typescript-eslint/parser",
    plugins: ["solid"],
    extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended", "plugin:solid/typescript"],
    parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module"
    },
    plugins: [
        "@typescript-eslint"
    ],
    root: true,
    rules: {
      "@typescript-eslint/no-unused-vars": ["warn", { "argsIgnorePattern": "^_" }]
    }
}
