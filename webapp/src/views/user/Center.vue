<template>
    <div id="center">
      <div id="top"></div>
      <div id="content">
        <h2><p>Uaer Center</p></h2>
        <p><strong style="color: green;">Welcome to the user center. You must first login to see user info!</strong></p>
        <p>email : {{ email }}</p>
        <p>username ：{{ username }}</p>
        <p>created_time : {{ created_time }}</p>
        <button id="submit" v-if="username == ''" @click="login">Login</button><br/>
        <button id="submit" v-if="username != ''" @click="update">UpdateAccount</button><br/>
        <button id="submit" v-if="username != ''" @click="deleteme">DeleteAccount</button><br/>
      </div>
      <div id="update" v-if="userupdate == true">
            <p>Account Update</p> 
              <input type="text" name="newname" placeholder="Newname" v-model="Newname"  required /><br/>
              <input type="text" name="newmail" placeholder="Newmail" v-model="Newmail"  required /><br/>
              <input type="password" name="newpassword" placeholder="Newpassword" v-model="Newpassword"  required/><br/>
              <input type="password" name="confirm_newpassword" placeholder="Confirm Newpassword" v-model="ConfirmNewpassword"  required/><br/>
              <button id="submit" @click="submitnow">UpdateNow</button>
            
      </div>
    </div>
</template>

<script>
import axios from 'axios'
import URLprefix from '../../config'
import auth from '../../utils/auth'
export default {
  name: 'center',
  data: function() {
    return {
      email: '',
      username: '',
      created_time: '',
      Newname: '',
      Newmail: '',
      Newpassword: '',
      ConfirmNewpassword: '',
      userupdate: false
    }
  },
  mounted: function() {
      if (sessionStorage.getItem('token')){
        axios.get(URLprefix + 'api/user_info', auth.getAuthHeader())
        .then((response) => {
            this.email =  response.data.current_user.email
            this.username =  response.data.current_user.username
            this.created_time =  response.data.current_user.created_at
            console.log(response.data.current_user)
            console.log(response.data.current_user.email)
        })
        .catch((e) => {
          console.log(e)
        })
      }
  },
  methods: {
    login() {
        window.location.reload ( true ); 
        this.$router.push('/a/access')
    },
    update() {
        this.userupdate = true
    },
    submitnow() {
        var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        var newname = this.Newname
        var newmail = this.Newmail
        var newpassword = this.Newpassword
        var confirm_newpassword = this.ConfirmNewpassword
        axios.post(URLprefix + 'api/user_update', {
            user_id: user_id,
            newname: newname,
            newmail: newmail,
            newpassword: newpassword,
            confirm_newpassword: confirm_newpassword
        })
        .then(response => {
          this.userupdate = false
          window.location.reload ( true )
        })
        .catch(e => {
          console.log(e)
        })
    },
    deleteme() {
        axios.get(URLprefix + 'api/user_delete', auth.getAuthHeader())
        .then((response) => {
            sessionStorage.removeItem('token')
            sessionStorage.removeItem('signin_user')
            window.location.reload ( true ); 
            this.$router.push('/')
        })
        .catch((e) => {
          console.log(e)
        })
        
    }
  }
}
</script>

<style scoped>
#center {
  line-height: 1.5rem;
}
#content {
  border: 1px solid green;
  padding: 2rem;
}
button {
    width: 7rem; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    #content {
      margin: 0.5rem auto;
      width: 95%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #top  {
        padding-top: 77px;
    } 
    #content,#update {
        margin: 0 auto;
        width: 72%;
    }
    #update {
      padding: 1rem 2rem;
    }
}
@media only screen and (min-width: 1000px) {
    #top  {
        padding-top: 77px;
    } 
    #content,#update {
        margin: 0 auto;
        width: 66%;
    }
    #update {
      padding: 1rem 2rem;
    }
}
</style>
