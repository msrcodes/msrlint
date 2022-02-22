module.exports = {
	extends: ['airbnb'],
	env: {
		browser: true,
		node: true,
	},
	rules: {
		semi: ['error', 'never'],
		indent: ['error', 'tab'],
		camelcase: ['error', {properties: 'never'}],
		'implicit-arrow-linebreak': 0,
		'func-style': [
			'error',
			'declaration', {allowArrowFunctions: true},
		],
		'no-tabs': 0,
		'linebreak-style': 0,
		'object-curly-spacing': ['error', 'never'],
		'object-curly-newline': 0,
	},
}