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
    <a class="py-2 px-4 border-r border-gray-900" href="/app/queue">Queue</a>
    <a class="py-2 px-4 border-r border-gray-900" href="/app/commands">
      Commands
    </a>
    <Menu as="div" class="relative ml-auto">
      <MenuButton class="py-2 px-4 border-gray-900 h-full">
        Profile
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
            :class="[
              'py-2',
              'px-4',
              'block',
              'w-max',
              { 'bg-gray-300': active },
            ]"
          >
            Some Link
          </a>
        </MenuItem>
        <MenuItem v-slot="{ active }">
          <a
            :class="[
              'py-2',
              'px-4',
              'block',
              'w-max',
              { 'bg-gray-300': active },
            ]"
          >
            Some Link
          </a>
        </MenuItem>
      </MenuItems>
    </Menu>
  </nav>
  <Queue />
</template>

<script>
import Queue from './components/Queue';
import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue';

export default {
  name: 'App',
  components: {
    Queue,
    Menu,
    MenuButton,
    MenuItems,
    MenuItem,
  },
};
</script>

<style></style>
