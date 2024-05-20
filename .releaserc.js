module.exports = {
  branches: [
    'release',
    {channel: 'release-candidate', name: 'release-candidate', prerelease: 'rc'},
  ],
  ci: false,
  plugins: [
    ['@semantic-release/changelog', {changelogFile: 'CHANGELOG.md'}],
    ['@semantic-release/commit-analyzer', {
        preset: 'angular',
        releaseRules: [
            {breaking: true, release: 'major'},
            {release: false, type: 'chore'},
            {release: false, type: 'ci'},
            {release: false, type: 'doc'},
            {release: false, type: 'no-release'},
            {release: false, type: 'release'},
            {release: false, type: 'test'},
            {release: false, type: 'wip'},
            {release: 'minor', type: 'feat'},
            {release: 'minor', type: 'refactor'},
            {release: 'minor', type: 'security'},
            {release: 'patch', type: 'fix'},
            {release: 'patch', type: 'update'},
        ],
    }],
    ['@semantic-release/exec', {
        prepareCmd: 'semantic-release-rust prepare \${nextRelease.version}',
        publishCmd: 'semantic-release-rust publish',
        verifyConditionsCmd: 'semantic-release-rust verify-conditions',
    }],
    ['@semantic-release/release-notes-generator', {
        preset: 'conventionalcommits',
        presetConfig: {
            types: [
                {hidden: false, section: ':water_buffalo: CHORE', type: 'chore'},
                {hidden: false, section: ':water_buffalo: UPDATES', type: 'update'},
                {hidden: false, section: ':vertical_traffic_light: CONTINUOUS INTEGRATION', type: 'ci'},
                {hidden: false, section: ':books: DOCUMENTATION', type: 'doc'},
                {hidden: false, section: ':sparkles: FEATURE', type: 'feat'},
                {hidden: false, section: ':bug: FIX', type: 'fix'},
                {hidden: false, section: ':wrench: REFACTOR', type: 'refactor'},
                {hidden: false, section: ':shield: SECURITY', type: 'security'},
                {hidden: false, section: ':dart: TEST', type: 'test'},
            ],
        },
    }],
    ['@semantic-release/git', {
        assets: ['CHANGELOG.md', 'README.md', 'doc/**', 'package.json', 'pnpm-lock.yaml'],
        message: 'release: ${nextRelease.version}.\n\n${nextRelease.notes}',
    }],
    ['@saithodev/semantic-release-backmerge', {
        branches: [{from: 'release', to: 'release-candidate'}],
    }],
  ],
}
