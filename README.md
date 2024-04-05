### xray-rust

use rust to write xray and add some new feature: such as read config dynamic, use less memory


xray 只定义 server 和 client 的接口，具体实现方式由不同类型的 stream 进行实现；
关于负载均衡？应该实现单downstream对应多upstream，这里lb的实现需要注入meteri进行测算；配置文件是否需要实现golang中viper那种动态下发？
如何利用cdn，upstream是否需要实现多路复用，是否需要对流量进行压缩？对端服务器是否需要实现xray中的回落？
客户端中dns请求是否需要实现 fake dns，或者直接交由系统实现？是否需要实现流量统计？
入站流量染色？还是基于某种规则进行路由？过滤？或者拦截？
例如：常常访问微软，但是分为国内国外流量，有时间会被重新定向到国外网站，所以需要将一个网页内的请求拆分出来，分别进行处理。