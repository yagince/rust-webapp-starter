import Vue from 'vue'
import Axios from 'axios'

const http = Axios.create({
  // for cors
  withCredentials: true
})
http.interceptors.response.use(
  (response) => {},
  (error) => {
    if (error.response.status === 400) {
      Vue.toasted.clear()
      Vue.toasted.error(error.response.data.message)
    } else if (error.response.status === 500) {
      Vue.toasted.clear()
      Vue.toasted.error(error.response.data.message)
    }
    return Promise.reject(error)
  }
)

export default http
