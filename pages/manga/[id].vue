<template>
  <div>
    <div class="mx-6 flex gap-4 h-100">
      <div class="flex-none aspect-[2/3] w-[20rem] h-full dark:bg-zinc-800 rounded-md overflow-hidden">
        <Cover
          v-if="manga"
          class="aspect-[2/3] h-auto w-full"
          :source="{ mangaId: manga.id, fileName: manga.relationships.find(r => r.type === 'cover_art')?.attributes?.fileName }"
        />
        <Skeleton v-else class="aspect-[2/3] w-full h-auto" />
      </div>

      <div v-if="manga" class="flex flex-col h-100">
        <h1 class="text-2xl font-bold">{{ manga.attributes.title.en }}</h1>
        <p class="grow mt-4 overflow-y-auto p-4 rounded-md shadow shadow-black dark:bg-zinc-800">{{
          manga.attributes.description.en }}</p>
      </div>
      <div v-else class="flex flex-col h-100">
        <Skeleton class="w-full h-[2rem]" />
        <Skeleton class="w-full h-[2rem]" />
      </div>
    </div>
    <div v-if="manga" class="px-6">
      <small class="px-2 py-0 rounded-md border border-green-500 bg-green-500/50 w-fit">
        {{ manga.attributes.contentRating }}
      </small>
    </div>
    <div>
      <Button color="primary" variant="solid" @click="startReading()">
        Start Reading
      </Button>
    </div>
    <div v-if="chapters" class="flex flex-col p-4">
      <div v-for="(chapter, i) in chapters.data" :key="chapter.id">
        <nuxt-link :to="`/manga/${manga.id}/${chapter.id}`">
          {{ `Chapter ${chapter.attributes.chapter ?? i + 1}` }}
          <template v-if="chapter.attributes.title?.length">
            {{ chapter.attributes.title }}
          </template>
        </nuxt-link>
      </div>
    </div>

    <Dialog v-model:open="openChapters">
      <DialogTrigger />
      <DialogContent>
        <DialogHeader>
          <DialogTitle></DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>
        <div class="p-6">
          <div v-if="loadingChapters" class="w-full h-[15rem] flex justify-center items-center">
            <LoaderCircle class="animate-spin" />
          </div>
          <div v-else class="flex flex-col max-h-[35rem] overflow-y-auto">
            <button v-for="chapter in firstChapters" :key="chapter.id"
              class="pb-2 mb-2 px-4 py-2 bg-cool-200 dark:bg-cool-700 border-l-2 border-l-primary-300 dark:border-l-primary-500 rounded"
              @click="navigateTo(`/manga/${manga.id}/${chapter.id}`)">
              <div class="flex justify-between">
                <strong>{{ `Ch. ${chapter.attributes.chapter}` }} {{ chapter.attributes.title }}</strong>
                <Flag :lang="chapter.attributes.translatedLanguage" />
              </div>
              <div class="md:flex md:justify-between">
                <small class="flex gap-2">
                  <Users class="w-3" v-if="hasGroups(chapter)" />
                  <span v-for="group in getGroups(chapter)">
                    {{ group.attributes.name }}
                  </span>
                </small>
                <small class="flex gap-2">
                  <User class="w-3" v-if="hasUsers(chapter)" />
                  <span v-for="user in getUsers(chapter)">
                    {{ user.attributes.username }}
                  </span>
                </small>
              </div>
            </button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { Users, User, LoaderCircle } from 'lucide-vue-next';
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog';
import { invoke } from '@tauri-apps/api/core';
import type { Chapter, Manga, Paginated, Volume } from '~/types';

const route = useRoute();
const id = route.params.id;
const manga: Ref<Manga> = ref(null as any);
const chapters: Ref<Paginated<Chapter>> = ref(null as any);

function getGroups(chapter: Chapter): { [k: string]: any } {
  return chapter.relationships.filter(r => r.type === "scanlation_group");
}
function getUsers(chapter: Chapter): { [k: string]: any } {
  return chapter.relationships.filter(r => r.type === "user");
}
function hasGroups(chapter: Chapter): boolean {
  return chapter.relationships.some(r => r.type === "scanlation_group");
}
function hasUsers(chapter: Chapter): boolean {
  return chapter.relationships.some(r => r.type === "user");
}

const firstChapters: Ref<Chapter[]> = ref([]);
const openChapters = ref(false);
const loadingChapters = ref(false);
async function startReading() {
  loadingChapters.value = true;
  let volumes = await invoke<{ [k: string]: Volume }>("get_volumes_and_chapters", { id: manga.value.id });

  const filters: any = {
    manga: manga.value.id,
    chapter: "1",
    limit: 100,
    order: { chapter: "asc" },
    contentRating: ['safe', 'suggestive', 'erotica', 'pornographic'],
    includes: ["scanlation_group", "user"]
  };

  if (Object.keys(volumes).includes("1")) {
    filters["volume"] = ["1"];
  }

  const response = await invoke<Paginated<Chapter>>("get_chapters", { filters });
  const result = response.data.sort((a, b) => ('' + a.attributes.translatedLanguage).localeCompare('' + b.attributes.translatedLanguage, undefined, { numeric: true, sensitivity: 'base' }));
  if (result.length === 1) {
    navigateTo(`/manga/${manga.value.id}/${result[0].id}`);
    return;
  }

  loadingChapters.value = false;
  firstChapters.value = result;
  openChapters.value = true;
}

onMounted(async () => {
  await invoke<Manga>("get_manga", { id, includes: ["cover_art"] })
    .then(response => manga.value = response);

  invoke<Paginated<Chapter>>("get_chapters", { filters: { manga: manga.value.id, translated_language: ["en"], order: { chapter: 'asc' } } })
    .then(response => {
      response.data = response.data.sort((a, b) => ('' + a.attributes.chapter).localeCompare('' + b.attributes.chapter, undefined, { numeric: true, sensitivity: 'base' }));
      chapters.value = response
    });
});
</script>
