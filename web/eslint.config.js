import antfu from '@antfu/eslint-config';

export default antfu({
  ignores: [
    'package.json',
    'bun.lock',
    'static/',
  ],
  stylistic: {
    indent: 2,
    semi: true,
    quotes: 'single',
  },
  svelte: true,
});
