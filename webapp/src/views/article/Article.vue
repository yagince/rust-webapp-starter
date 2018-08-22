<template>
    <div id="article_id">
        <mnav id="mnav"></mnav>
                    <div id="content">
                        <div id="title">
                            <h2> {{ article.title }} </h2> 
                            <span id="info">{{ article.category }}</span> • 
                            <span id="info"><a :href="'/a/user/' + article.user_id">user_id : {{ article.user_id }}</a></span> •   
                            <span id="info">{{ article.created_at }}</span>  
                        </div>
                        <div id="body">content : {{ article.body }}</div>
                    </div>
    </div>
</template>

<script>
import axios from 'axios'
import URLprefix from '../../config'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'article_id',
    components: {
        "mnav": Mnav
    },
    data: function() {
        return {
            article: ''
        }
    },
    mounted: function() {
    // let article_id = this.$route.params.id
    // console.log(article_id)
    axios.get(URLprefix + 'api/article/' + this.$route.params.id)
      .then((response) => {
        this.article = response.data.article
        console.log(response.data.article)
      })
      .catch((e) => {
        console.log(e)
      })
  }
}
</script>

<style scoped>
#article_id {
    line-height: 1.5rem;
    background-color: #ffffff;
}
a {
    color: #0541af;
}
#content #title {
    margin-top: 2px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#title h2 { 
    padding-bottom: 0.3rem;
}
#title #info {
    display: inline-block;
    font-size: 14px;
}
#body {
    margin: 1rem auto;
    word-break: break-all;
    word-wrap: break-word;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
  }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
}
</style>