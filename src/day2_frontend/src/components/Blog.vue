<template>
    <h2 class="text-blue-600 font-semibold">Blogs</h2>
    <div class="w-100 flex flex-row-reverse">
        <button class="bg-red-200 p-2 rounded-sm" @click="fetchBlogs">Fetch Blogs</button>
    </div>
   
    <div class="grid grid-cols-4 py-2">
        <div class="bg-stone-200 m-2" v-for="blog in blogs" :key="blog.id">
            <p>{{ blog }}</p>
        </div>
    </div>
    <input class="border-1 border-red-900 bg-yellow-100" type="text" v-model="newBlog" />
    {{ newBlog }}
    <button class="bg-red-200 p-2 rounded-sm" @click="addBlog">Add blog</button>
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
                await day2_backend.add_blog(this.newBlog);
                this.newBlog = '';
                this.fetchBlogs();
            }
        },
        async mounted(){
            this.fetchBlogs();
        }
    };
</script>