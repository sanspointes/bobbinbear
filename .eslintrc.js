const PRETTIER_CONFIG = {
    endOfLine: 'auto',
    singleQuote: true,
    tabWidth: 4,
};

module.exports = {
    env: {
        browser: true,
    },
    settings: {
        'import/resolver': {
            typescript: true,
        },
    },
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint', 'solid', 'unicorn', 'import'],
    extends: [
        'eslint:recommended',
        'plugin:import/recommended',
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
