# Changelog

## [0.0.16](https://github.com/zhanglun/pavo/compare/v0.0.15...v0.0.16) (2026-08-02)


### Bug Fixes

* **linux:** 修复托盘图标黑框、窗口定位与拖动失效 ([29bdc79](https://github.com/zhanglun/pavo/commit/29bdc79e779d22fdf62fa7590f83cd7c051c81b8))
* **menu:** 防止更多菜单被窗口边界裁切 ([85dbb04](https://github.com/zhanglun/pavo/commit/85dbb04d4b11ff50649be51f49e5106d16632327))

## [0.0.15](https://github.com/zhanglun/pavo/compare/v0.0.14...v0.0.15) (2026-07-24)


### Features

* 前端整体从 Svelte 重构为 React，重建每日册页、历史档案与收藏册页
* **today:** 落地非对称日历日期版式，地区入口重做为摄影联页，版权说明就地展开
* **history:** 历史档案按日期分组，同一天有几张显几张
* **ui:** 用 lucide-react 替换 Unicode 字符图标，更新应用与跨平台托盘图标
* 主题偏好持久化
* 强制刷新壁纸数据，窗口激活时自动刷新


### Bug Fixes

* **bing:** reqwest 客户端优先 http 代理，修复多地区数据被统一为中文区 ([75f7aee](https://github.com/zhanglun/pavo/commit/75f7aeeea6c0ca24746a3350334e31005ee1a085))
* **scheduler:** merge 合并键加入日期，修复同一天多图丢失 ([705d429](https://github.com/zhanglun/pavo/commit/705d429b9ce42b8f33d45cac489aa138cb1f9b22))
* **today:** 今日各地取每个地区最新一张，修复时区错配丢图 ([e3d4c4d](https://github.com/zhanglun/pavo/commit/e3d4c4d1086e37cc73be20deccd5e5a69335195a))
* **today:** 跨午夜自动刷新今日页，避免显示昨天的日期与壁纸 ([cf90ec7](https://github.com/zhanglun/pavo/commit/cf90ec75a393ac0ca015e100f133bd41fedea20c))
* **today:** 日期标签固定显示系统当前日期，不跟随壁纸 startdate ([5e81e8a](https://github.com/zhanglun/pavo/commit/5e81e8aa80bbad1ca6f5ae6814f8acd6edb6d292))
* **favorites:** 收藏按时间降序排列，最近收藏的排前面 ([be1b3cd](https://github.com/zhanglun/pavo/commit/be1b3cd3ed2437275cd0ed57f8a9512b44c4d741))
* **history:** 合并同图多地区为一条，聚合地区名显示 ([c17897c](https://github.com/zhanglun/pavo/commit/c17897cc9464f07fa5c71170eb16ee9ca9c8b205))
* 补齐关键操作反馈与启动稳定性 ([da9c790](https://github.com/zhanglun/pavo/commit/da9c79007832b21f89b0118904ef897a788cdc77))
* 消除切换地区的主体抖动，重做地区栏选中态 ([1fa4387](https://github.com/zhanglun/pavo/commit/1fa43870cef69aa0bfa93c4a6baa40d5986034bf))

## [0.0.14](https://github.com/zhanglun/pavo/compare/pavo-v0.0.13...pavo-v0.0.14) (2026-05-01)


### Bug Fixes

* 统一 Tauri Rust crate 和 NPM API 包版本到 2.9.x ([4d7d566](https://github.com/zhanglun/pavo/commit/4d7d566ea21f6aa616580117628fdfc2508f5128))

## [0.0.13](https://github.com/zhanglun/pavo/compare/pavo-v0.0.12...pavo-v0.0.13) (2026-05-01)


### Features

* :construction: set and download from Bing ([dafd20c](https://github.com/zhanglun/pavo/commit/dafd20c009969d2e1a4279f40db9e0948cfacff6))
* add About page ([4e784d1](https://github.com/zhanglun/pavo/commit/4e784d1800d097acbc379c2693ea5a9dbb9a396e))
* add backend daily update and wallpaper queries ([a3dce2b](https://github.com/zhanglun/pavo/commit/a3dce2b650de28394842c158d65e03965cfbd640))
* add bing daily wallpaper config model ([5866047](https://github.com/zhanglun/pavo/commit/5866047192205c9c8b63ba541becf19834ffb03a))
* add DailyImage ([bed3783](https://github.com/zhanglun/pavo/commit/bed378352eb4fb47ca7469843852fc68cb19343b))
* add desktop layer ([448ebb6](https://github.com/zhanglun/pavo/commit/448ebb615a56ed848810fb30386952b454f14b93))
* add logo ([55c389c](https://github.com/zhanglun/pavo/commit/55c389c47a9d58735733cf4905f6fa838abb36cd))
* add meta when set wallpaper ([983c253](https://github.com/zhanglun/pavo/commit/983c253a5dfb24e00ebb1e13184ff2249f27e3bf))
* add rotate source into config ([709b4f0](https://github.com/zhanglun/pavo/commit/709b4f0d9fffff7408f9b9ce733541511d613169))
* add show layer config ([9a82cda](https://github.com/zhanglun/pavo/commit/9a82cdaed0a19a2f7927845631b07f20261358fb))
* add Skeleton ([ec2d7fb](https://github.com/zhanglun/pavo/commit/ec2d7fbaa09eae92430106666808c949b42af0e3))
* add switch menu item ([2af5e93](https://github.com/zhanglun/pavo/commit/2af5e9359a5a74d2cbb112aaa8b23e272e880edf))
* add tailwindcss support ([ff45b7e](https://github.com/zhanglun/pavo/commit/ff45b7e3b338ea8abe27162b99f7f19533f344e2))
* add tauri plugin log ([4f20849](https://github.com/zhanglun/pavo/commit/4f20849cccc50ebff535db2896412c672a9c4d2c))
* add tray menu ([a2d72ef](https://github.com/zhanglun/pavo/commit/a2d72ef10ad691ffe27dc2118b6deaa2d5645130))
* add Update async runtime ([ca03259](https://github.com/zhanglun/pavo/commit/ca03259fbbc97ad439f89924caeeb1594160f83f))
* add Updater ([d02329d](https://github.com/zhanglun/pavo/commit/d02329dc04ae8e2010e9d684d46de75daf4cddfc))
* build bing daily wallpaper app surfaces ([5d2a7ba](https://github.com/zhanglun/pavo/commit/5d2a7bad53dc707dce4d58b488e207bbca2bd317))
* cache data ([779659a](https://github.com/zhanglun/pavo/commit/779659aecc0e36cd451d6442679a82f76c4d9ffe))
* close not exit but minimize ([be2a064](https://github.com/zhanglun/pavo/commit/be2a0641953411dbfd41d2a2a1ef5b61614c50e1))
* cmd set auto_save ([f3c409b](https://github.com/zhanglun/pavo/commit/f3c409bb080f63bf8c055b1ee70d603e6ac46056))
* compile tray icon for window ([64353af](https://github.com/zhanglun/pavo/commit/64353af8cbcbc5bb099eedb9be0a158038320b17))
* download wallpaper from url ([47bf1fc](https://github.com/zhanglun/pavo/commit/47bf1fc863bda3d0f1b356a33c10b83fec5fe139))
* fix photo aspect ([93635cc](https://github.com/zhanglun/pavo/commit/93635cc71415f54da29b0b7a6b9ae2a9ebbacfea))
* fixed position with tray icon ([5471d31](https://github.com/zhanglun/pavo/commit/5471d31d08e1ec8e67db8b408aeb85f3771a173a))
* get bing wallpaper 2 pages ([a869912](https://github.com/zhanglun/pavo/commit/a86991234115d4568260a7680d1e491e32b6d680))
* **hero:** 今日壁纸轮播展示 ([379850b](https://github.com/zhanglun/pavo/commit/379850bc69c9d28bb4a165b7e1b5afd97576d6b0))
* hide docker icon on MacOS ([44c22b1](https://github.com/zhanglun/pavo/commit/44c22b1c47200057cf1c0b6ec566f2c594dc6d81))
* keep shuffe when encounter download problem ([146689a](https://github.com/zhanglun/pavo/commit/146689a00487f494f03f9431ce8464e63f50c5c0))
* merge all regions data ([d2863e7](https://github.com/zhanglun/pavo/commit/d2863e73b964bffb069ded08e740c8ab30fdbf8a))
* open source in browser ([f83aab4](https://github.com/zhanglun/pavo/commit/f83aab426ee62f51bb00ddbf53f21d61168e887a))
* pexel pagination ([07a8357](https://github.com/zhanglun/pavo/commit/07a83575acd9181610134b5cce2167ecd09ff9b8))
* remove tauri-plugin-positioner ([656d8b9](https://github.com/zhanglun/pavo/commit/656d8b98de3b0b546a37e5209cf219784329f090))
* request Pexels ([c3d7180](https://github.com/zhanglun/pavo/commit/c3d7180df07876c6ad365c4341cec1848011e091))
* reveal log ([b3828e6](https://github.com/zhanglun/pavo/commit/b3828e66cc399e95aa92714fd16243deb2b6b11a))
* **rotation:** 自动轮播壁纸功能 ([771ed82](https://github.com/zhanglun/pavo/commit/771ed82b51d60ffed5f38bfe91e3ff7301db1d75))
* **scheduler:** 新增 get_today_collection 命令返回今日各地区壁纸合集 ([4e7c575](https://github.com/zhanglun/pavo/commit/4e7c575bb7b800f3a0d65b21cb3581613463997a))
* set underlay position to right corner ([66fff09](https://github.com/zhanglun/pavo/commit/66fff09e5219f0af9b9839e203734492e0771e6e))
* single instance ([d458aee](https://github.com/zhanglun/pavo/commit/d458aeecc43e2b4c5c51d5d64baca0129ae08fac))
* support rotate randomly ([8a8287f](https://github.com/zhanglun/pavo/commit/8a8287f7e442a832e57685622de15367eb270f5b))
* support stop auto rotate ([eb9a9e1](https://github.com/zhanglun/pavo/commit/eb9a9e19f93ed179f3661262da292c2514b2fcd4))
* **tray:** 窗口定位到托盘图标附近 ([708b425](https://github.com/zhanglun/pavo/commit/708b4255c9e44932ebe11e4eec2bce8064dbb20d))
* **ui:** :lipstick: scroll in tab content ([49ae687](https://github.com/zhanglun/pavo/commit/49ae687ff8ff141daaf0eed3459f775f7f943ad8))
* **ui:** 无边框主窗口 ([ab50999](https://github.com/zhanglun/pavo/commit/ab5099938412d3994ac58d2fd206cccab6bc8971))
* **ui:** 首页重构为单页面滚动布局 ([9e36433](https://github.com/zhanglun/pavo/commit/9e36433f64e2958ba33852c95718fc924c7bd928))
* update config ([8dae5b4](https://github.com/zhanglun/pavo/commit/8dae5b42235772dc581908e867fee36041d4aee1))
* update meta in underlayer window when wallpaper changed ([faee570](https://github.com/zhanglun/pavo/commit/faee5704f91af3aa7d9c1f2f291a32d8b9974324))
* update sidebar ([3cfd889](https://github.com/zhanglun/pavo/commit/3cfd88988a763adfccd25ac38cf08e8d0fc64b5d))
* upgrade tauriv2 ([aa6a27b](https://github.com/zhanglun/pavo/commit/aa6a27b5129f8d552f1b30d28544fe3c4b59bbbd))
* vibe coding ([8801d68](https://github.com/zhanglun/pavo/commit/8801d6838d91714ac862b09fb98de2b664c331f1))
* view photo in window ([17a19ea](https://github.com/zhanglun/pavo/commit/17a19ea976d50673c8353e830d727011971080d3))


### Bug Fixes

* access app_handler in cmd ([b07602a](https://github.com/zhanglun/pavo/commit/b07602a06470d2b0423482a57ba346f11d26edae))
* add once_cell ([f869333](https://github.com/zhanglun/pavo/commit/f869333fb41fd0522338e10e76cae0611bdbeff7))
* **backend:** toggle 修复、错误处理、死代码清理、自动更新确认 ([146711c](https://github.com/zhanglun/pavo/commit/146711cdfbbf91143c042a926aee121a92685501))
* background progress no data ([e1c1e29](https://github.com/zhanglun/pavo/commit/e1c1e29f51b1384f8c609f20861ff037b4825bac))
* clean code ([ca15368](https://github.com/zhanglun/pavo/commit/ca15368d8570efb285ae92e269dfa28e39909de1))
* clean pexels ([a72098e](https://github.com/zhanglun/pavo/commit/a72098e40d132ae6cc739a8488a395ac5538a985))
* clone mpsc sender to tray callback ([286d92d](https://github.com/zhanglun/pavo/commit/286d92d741ec4e93cf852d7dcdcfef9e972c186d))
* compile error ([12896ca](https://github.com/zhanglun/pavo/commit/12896ca7771916fef5ccc94fc4affe4f29e54b6a))
* **config:** 修复读写竞态，添加全局 Mutex ([7a8b2ea](https://github.com/zhanglun/pavo/commit/7a8b2ea28c5e2f7a0e0e3946ba5d95457c072a10))
* Error `tauri.conf.json` error on `bundle > createUpdaterArtifacts`: "true" is not valid under any of the schemas listed in the 'anyOf' keyword ([beccc91](https://github.com/zhanglun/pavo/commit/beccc91c0904be43591d614de4bc002d77c43daf))
* error path ([aeedd20](https://github.com/zhanglun/pavo/commit/aeedd204bcb13078cf2c5034ddda1863a6c2f670))
* fix auto rotate ([d1c2dba](https://github.com/zhanglun/pavo/commit/d1c2dbae1f7fc142aeaec845dd6fc848dc388e19))
* fix cache lock blocking ([11f84c1](https://github.com/zhanglun/pavo/commit/11f84c1a9a41121c73059bfa380364ed7547fda9))
* fix cache problem ([51c7db9](https://github.com/zhanglun/pavo/commit/51c7db978c5da98ffbc85ed3c2c6df07aa64b6a9))
* fix compiler error ([a5d4cc5](https://github.com/zhanglun/pavo/commit/a5d4cc5fdf33ad5aab2ddb859194a38e2d62985c))
* fix config binding ([85e00c6](https://github.com/zhanglun/pavo/commit/85e00c649509934cf3e6949bc438f3b7790b4f58))
* fix dark mode ([5d2af7f](https://github.com/zhanglun/pavo/commit/5d2af7fae7f9994bb22085138ed753994e3df61f))
* fix error ([31104d0](https://github.com/zhanglun/pavo/commit/31104d09e23c4356b8917f09b963bb11aa4e2cfb))
* fix error ([5a0d134](https://github.com/zhanglun/pavo/commit/5a0d1344bfdadab2aebe36b6007dd219e22041dd))
* fix grammer error ([69cef6a](https://github.com/zhanglun/pavo/commit/69cef6aa41ba2f26b4dcf14fca5b236d9eebfec4))
* fix pexel rotate ([de44e33](https://github.com/zhanglun/pavo/commit/de44e33c5670e5f06a0fbf993ec64b6e7c57e737))
* fix SampleRange&lt;_&gt; in Win11 ([95ff60e](https://github.com/zhanglun/pavo/commit/95ff60e8d22db39d9cea763499fc43f053ae9a68))
* fix version ([6a8d97b](https://github.com/zhanglun/pavo/commit/6a8d97b9cc597a14906afebaef848ac48160a4ca))
* index out of bounds ([b098fe4](https://github.com/zhanglun/pavo/commit/b098fe483a575e5838bd54298b392e2cd62489fd))
* init underlayer with config data ([b89fb40](https://github.com/zhanglun/pavo/commit/b89fb4008e640322b496ad638b300f29f700a53c))
* migrate tray-icon in tauri v2 ([1234f4b](https://github.com/zhanglun/pavo/commit/1234f4b4a646373e2a1f9d3145e165a71d3c88d5))
* next photo ([ae09bd7](https://github.com/zhanglun/pavo/commit/ae09bd7deb365e3d2b2bfcf7fd41900315983e33))
* open external link in default browser ([2b45144](https://github.com/zhanglun/pavo/commit/2b4514458e718209625c2c7eda64a6df259c015b))
* **plugins:** 删除重复的 single_instance 插件注册 ([db0fdb3](https://github.com/zhanglun/pavo/commit/db0fdb392d289b75b5252bebcd4073a1f7db1070))
* pnpm-lock ([a3bef66](https://github.com/zhanglun/pavo/commit/a3bef660245f7206405c4f6403ee62ea3f344ba1))
* service test code ([d014466](https://github.com/zhanglun/pavo/commit/d0144662d960b42ccefc78dd295fe88bc7c9fac7))
* tray icon ([6cb6ad0](https://github.com/zhanglun/pavo/commit/6cb6ad0d4abed564269e3bff2fb3ab62cf5ba943))
* tsconfig.json ([6bb8ba7](https://github.com/zhanglun/pavo/commit/6bb8ba7dc1b0046080efada6f2d86fabdafadbfe))
* unminimize after minimized ([5db7d6c](https://github.com/zhanglun/pavo/commit/5db7d6c4afacdf8c32cde064420941aef140650c))
* update with message dialog ([30ca062](https://github.com/zhanglun/pavo/commit/30ca0621d9dd8378ebc292816694137b22195c8d))
* use showfile to open file ([6feeb49](https://github.com/zhanglun/pavo/commit/6feeb498c8ddd5981c55254c3a05cf7432e12d32))
* v1Compatible ([9c2fafb](https://github.com/zhanglun/pavo/commit/9c2fafbda1e9273a2fa786ec4f0f0b25de92d87a))
* vite output path ([d48cba5](https://github.com/zhanglun/pavo/commit/d48cba5b2de1819ab74356b805581ebb13a437e7))
* Warn Legacy v1 compatible updater is deprecated ([eb71d3b](https://github.com/zhanglun/pavo/commit/eb71d3bac0b1ca4842dc15b1963596fae62e9b67))
