<template>
  <div>
    <h2 class="text-blue-600">Tell me something about yourself, my Blog</h2>
    <div class="w-100 flex flex-row-reverse">
      <button @click="fetchPosts" class="bg-blue-600 rounded text-white p-4">
        Fetch
      </button>
    </div>
    <div class="grid mx-6 gap-4 my-4">
      <div
        v-for="(post, index) in posts"
        class="drop-shadow-xl bg-stone-300 p-4"
      >
        <p>id: {{ index }}</p>
        <p>{{ post }}</p>
      </div>
    </div>
    <div class="flex justify-center flex-col">
      <input
        v-model="newPost"
        class="border-2 border-blue-600 p-4"
        type="text"
      />
      <button @click="addPost" class="bg-blue-600 rounded text-white p-4">
        Add
      </button>
    </div>
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
  async mounted() {
    this.fetchPosts();
  },
};
</script>
