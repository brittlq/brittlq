<template>
  <nav
    class="
      flex flex-row
      justify-items-center
      col-span-4
      border-b-2 border-gray-900
      bg-gray-200
    "
  >
    <h1 class="py-2 px-4 border-r border-gray-900 text-xl font-mono w-1/6">
      BrittlBot/BrittlQ
    </h1>
    <router-link
      class="py-2 px-4 border-r border-gray-900 hover:bg-gray-300"
      to="/party-queue"
      >Queue</router-link
    >
    <router-link
      class="py-2 px-4 border-r border-gray-900 hover:bg-gray-300"
      to="/commands"
      >Commands</router-link
    >
    <router-link
      class="py-2 px-4 border-r border-gray-900 hover:bg-gray-300"
      to="/obs"
      >OBS Controls</router-link
    >
    <Menu as="div" class="relative ml-auto" v-slot="{ open }">
      <MenuButton
        :class="[
          'py-2',
          'px-4',
          'h-full',
          'hover:bg-gray-300',
          { 'bg-gray-300': open },
        ]"
      >
        Profile
        <fa-icon v-if="open" :icon="['fas', 'chevron-up']" />
        <fa-icon v-else :icon="['fas', 'chevron-down']" />
      </MenuButton>
      <MenuItems
        class="
          absolute
          right-0
          origin-top-right
          divide-y divide-gray-100
          bg-gray-200
          border-gray-900 border border-t-0
          z-50
        "
      >
        <MenuItem v-slot="{ active }" v-if="!hasToken">
          <a
            :href="twitchOauthUri"
            :class="[
              'py-2',
              'px-4',
              'block',
              'w-full',
              'min-w-max',
              { 'bg-gray-300': active },
            ]"
          >
            Connect to Twitch
            <fa-icon :icon="['fab', 'twitch']" />
          </a>
        </MenuItem>
        <MenuItem v-slot="{ active }" v-else-if="hasToken">
          <button
            :class="[
              'py-2',
              'px-4',
              'block',
              'w-full',
              'min-w-max',
              { 'bg-gray-300': active },
            ]"
            @click="logoutTwitch"
          >
            Disconnect from Twitch
            <fa-icon :icon="['far', 'times-circle']" />
          </button>
        </MenuItem>
      </MenuItems>
    </Menu>
    <button
      type="button"
      @click="isChatExpanded = !isChatExpanded"
      class="px-4"
    >
      <fa-icon
        :icon="['fas', 'angle-double-right']"
        :aria-label="toggleChatLabel"
        :title="toggleChatLabel"
        :class="[
          'transition-transform',
          'transform',
          'delay-300',
          { 'rotate-180': !isChatExpanded },
        ]"
      />
    </button>
  </nav>
</template>

<script lang="ts">
import { useTwitchStore } from '@/store/twitch';
import { useTwitchChatStore } from '@/store/twitch-chat';
import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue';
import { storeToRefs } from 'pinia';
import { computed, defineComponent } from 'vue';
export default defineComponent({
  components: {
    Menu,
    MenuButton,
    MenuItems,
    MenuItem,
  },
  setup() {
    const twitchChatStore = useTwitchChatStore();
    const twitchStore = useTwitchStore();
    const toggleChatLabel = computed(() =>
      twitchChatStore.isChatExpanded ? 'Hide Twitch Chat' : 'Show Twitch Chat'
    );
    const { isChatExpanded } = storeToRefs(twitchChatStore);
    const { hasToken, twitchOauthUri } = storeToRefs(twitchStore);
    const logoutTwitch = () => {
      twitchStore.token = '';
    };
    return {
      toggleChatLabel,
      isChatExpanded,
      logoutTwitch,
      hasToken,
      twitchOauthUri,
    };
  },
});
</script>

<style></style>
