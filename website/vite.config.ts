import { defineConfig } from 'vite'

// 部署到 GitHub Project Pages (https://zhanglun.github.io/pavo/) 时，
// 生产构建使用 '/pavo/' 作为 base；本地 dev 保持 '/' 以便直接访问。
export default defineConfig(({ mode }) => ({
  base: mode === 'production' ? '/pavo/' : '/',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
}))
