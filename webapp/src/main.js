import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Axios from './axios'
import Toasted from 'vue-toasted'
import './registerServiceWorker'

Vue.config.productionTip = false
Vue.prototype.$http = Axios
Vue.use(Toasted)

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
