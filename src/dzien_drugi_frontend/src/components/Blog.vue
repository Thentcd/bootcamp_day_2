<template>
  <button @click="fetchPosts">Fetch</button>
  <div>
    siema blog! {{ posts }} <input v-model="newPost" type="text" />
    <button @click="addPost">Add</button>
  </div>
</template>

<script>
import { dzien_drugi_backend } from 'declarations/dzien_drugi_backend/index';
export default {
  data() {
    return {
      posts: [],
      newPost: '',
    };
  },
  methods: {
    async addPost() {
      await dzien_drugi_backend.add_post(this.newPost);
    },
    async fetchPosts() {
      this.posts = await dzien_drugi_backend.read_posts();
    },
  },
};
</script>
