import { writable } from 'svelte/store'

const PROJECT_NAME = writable('sample')
const CONTENTS = writable(['ブログ', 'タグ', 'ピックアップタグ'])

export { PROJECT_NAME, CONTENTS }
