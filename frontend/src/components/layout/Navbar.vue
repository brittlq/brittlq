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
    >
      Queue
    </router-link>
    <router-link
      class="py-2 px-4 border-r border-gray-900 hover:bg-gray-300"
      to="/commands"
    >
      Commands
    </router-link>
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
        "
      >
        <MenuItem v-slot="{ active }">
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
        <MenuItem v-slot="{ active }">
          <a
            :class="[
              'py-2',
              'px-4',
              'block',
              'w-full',
              'min-w-max',
              { 'bg-gray-300': active },
            ]"
          >
            Some Link
          </a>
        </MenuItem>
      </MenuItems>
    </Menu>
    <button type="button" @click="toggleChat" class="px-4">
      <fa-icon
        :icon="['fas', 'angle-double-right']"
        :aria-label="chatOpen ? 'Hide Chat' : 'Show Chat'"
        :class="[
          'transition-transform',
          'transform',
          { 'rotate-180': !chatOpen },
        ]"
      />
    </button>
  </nav>
</template>

<script>
import { mapGetters, mapMutations, mapState } from 'vuex';
import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue';
import { TOGGLE_CHAT_SIDEBAR } from '../../store/mutations';
export default {
  components: {
    Menu,
    MenuButton,
    MenuItems,
    MenuItem,
  },
  computed: {
    ...mapGetters(['twitchOauthUri']),
    ...mapState({
      chatOpen: 'chatSidebarOpen',
    }),
  },
  methods: {
    ...mapMutations({
      toggleChat: TOGGLE_CHAT_SIDEBAR,
    }),
  },
};
</script>

<style></style>
