const PRETTIER_CONFIG = {
    endOfLine: 'auto',
    singleQuote: true,
    tabWidth: 4,
};

module.exports = {
    env: {
        browser: true,
        es2024: true,
    },
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint', 'solid', 'unicorn'],
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:solid/typescript',
        'plugin:prettier/recommended',
    ],
    parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
    },
    root: true,
    rules: {
        '@typescript-eslint/no-unused-vars': [
            'warn',
            { argsIgnorePattern: '^_' },
        ],
        'prettier/prettier': ['warn', PRETTIER_CONFIG],
    },
};
