<template>
    <button @click="fetchBlogs">Fetch Blogs</button>
    <div>Siema</div>
    {{ blogs }}
    <input type="text" v-model="newBlog" />
    {{ newBlog }}
    <button @click="addBlog">Add blog</button>
</template>

<script>
import { day2_backend } from 'declarations/day2_backend/index';

    export default {
        data(){
            return{
                blogs: [],
                newBlog: ''
            }
        },
        methods: {
            async fetchBlogs(){
                const response = await day2_backend.read_blogs();
                this.blogs = response;
            },
            async addBlog(){
                await day2_backend.add_blogs(this.newBlog);
                this.newBlog = '';
                this.fetchBlogs();
            }
        },
    };
</script>