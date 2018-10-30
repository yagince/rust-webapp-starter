import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Axios from './axios'
import Toasted from 'vue-toasted'
import './registerServiceWorker'
import VueMaterial from 'vue-material'
import 'vue-material/dist/vue-material.min.css'

Vue.config.productionTip = false
Vue.prototype.$http = Axios
Vue.use(Toasted)
Vue.use(VueMaterial)

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
