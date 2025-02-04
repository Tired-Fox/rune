<template>
  <UIcon :name="`i-flag-${region}-4x3`" :title="regionName.length ? `${langName} (${regionName})` : langName" />
</template>
<script setup lang="ts">
import { langToCountry } from '~/util/lang';

const props = defineProps<{ lang: string }>()

let lang = '';
let langName = '';
let region = '';
let regionName = '';

if (props.lang.includes('-')) {
  const locale = new Intl.Locale(props.lang);
  lang = locale.language;
  langName = '' + (new Intl.DisplayNames(['en'], { type: 'language' })).of(lang);
  region = '' + locale.region?.toLowerCase();
  regionName = '' + (new Intl.DisplayNames(['en'], { type: 'region' })).of(locale.region);
} else {
  lang = props.lang;
  langName = '' + (new Intl.DisplayNames(['en'], { type: 'language' })).of(lang);
  region = '' + langToCountry[lang];
  regionName = '';
}
</script>
