export const getCookie = (name: string) => {
	const value = `; ${document.cookie}`
	const parts = value.split(`; ${name}=`)
	if (parts.length === 2) return parts.pop().split(';').shift()
}

export const deleteCookie = (name: string) => {
	document.cookie = `${name  }=;expires=Thu, 01 Jan 1970 00:00:01 GMT;`
}
