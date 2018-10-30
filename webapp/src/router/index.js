import Vue from 'vue'
import Router from 'vue-router'
import App from '@/App'
import Home from '@/views/home/Home'
import Wiki from '@/views/wiki/Wiki'
import Article from '@/views/article/Article'
import New from '@/views/new/New'
import Access from '@/views/user/Access'
import SignUp from '@/views/user/SignUp'
import Center from '@/views/user/Center'
import About from '@/views/about/About'
import NotFound from '@/views/notfound/NotFound'

Vue.use(Router)

export default new Router({
  mode: 'history',
  linkActiveClass: 'is-active',
  routes: [
    { path: '/', component: App, children: [
      { path: '', name: 'home', component: Home},
      { path: 'a/wiki', name: 'wiki', component: Wiki },
      { path: 'a/article/:id', name: 'article', component: Article },
      { path: 'a/new', name: 'new', component: New },
      { path: 'a/access', name: 'access', component: Access },
      { path: 'a/signup', name: 'signup', component: SignUp },
      { path: 'a/user/:id', name: 'user', component: Center },
      { path: 'a/about', name: 'about', component: About },
    ]},
    { path: '*', name: 'notfound', component: NotFound }
  ]
})
