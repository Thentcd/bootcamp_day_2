<script setup>
import { ref } from 'vue';
import { dzien_drugi_backend } from 'declarations/dzien_drugi_backend/index';
import Blog from './components/Blog.vue';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const surname = target.querySelector('#surname').value;
  await dzien_drugi_backend.greet(name, surname).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name and surname: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <input id="surname" alt="Surname" type="text" />
      <button type="submit">Greet Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
    <Blog />
  </main>
</template>
