import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => {
    const savedState = localStorage.getItem('user-state')
    return savedState ? JSON.parse(savedState) : {
      user: null,
      isAuthenticated: false
    }
  },

  actions: {
    setUser(userData) {
      this.user = userData
      this.isAuthenticated = true
      this.saveState()
    },

    clearUser() {
      this.user = null
      this.isAuthenticated = false
      this.saveState()
    },

    saveState() {
      localStorage.setItem('user-state', JSON.stringify({
        user: this.user,
        isAuthenticated: this.isAuthenticated
      }))
    }
  }
}) 