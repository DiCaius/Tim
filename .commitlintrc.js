const ERROR_CODE = 2
const HEADER_MAX_LENGTH = 100

module.exports = {
  extends: '@commitlint/config-angular',
  ignores: [
    message =>
        message ? /^release: \d+\.\d+\.\d+(-(rc)\.\d+)?\./.test (message) :
        /* otherwise */ false,
  ],
  rules: {
    'body-case': [ERROR_CODE, 'always', 'sentence-case'],
    'body-full-stop': [ERROR_CODE, 'always'],
    'body-leading-blank': [ERROR_CODE, 'always', 'sentence-case'],
    'footer-leading-blank': [ERROR_CODE, 'always'],
    'header-max-length': [ERROR_CODE, 'always', HEADER_MAX_LENGTH],
    'scope-enum': [ERROR_CODE, 'always', [
        'lib/hkt_macro',
    ]],
    'subject-case': [ERROR_CODE, 'always', 'sentence-case'],
    'subject-full-stop': [ERROR_CODE, 'always', '.'],
    'type-case': [ERROR_CODE, 'always', 'lower-case'],
    'type-enum': [ERROR_CODE, 'always', [
        'chore',
        'ci',
        'doc',
        'feat',
        'fix',
        'no-release',
        'refactor',
        'release',
        'security',
        'test',
        'update',
        'wip',
    ]],
  },
}
