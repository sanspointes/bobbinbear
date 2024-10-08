module.exports = {
    env: {
        browser: true,
    },
    settings: {
        'import/resolver': {
            typescript: true,
            alias: {
                map: [ 
                    [ '@', './src']
                ]
            }
        },
    },
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint', 'solid', 'unicorn', 'import'],
    extends: [
        'eslint:recommended',
        'plugin:import/recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:solid/typescript',
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
    },
};
