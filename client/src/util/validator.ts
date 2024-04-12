const EMAIL_REGEX = /^(([^<>()[\]\.,;:\s@\"]+(\.[^<>()[\]\.,;:\s@\"]+)*)|(\".+\"))@(([^<>()[\]\.,;:\s@\"]+\.)+[^<>()[\]\.,;:\s@\"]{2,})$/i

export const validateEmail = (email: string): boolean => {
	return EMAIL_REGEX.test(email)
}

interface PasswordValidationResult {
    lengthValid: boolean;
    hasUppercase: boolean;
    hasSpecialCharacter: boolean;
}

export const validatePassword = (password: string): PasswordValidationResult => ({
	lengthValid: password.length >= 8,
	hasUppercase: /[A-Z]/.test(password),
	hasSpecialCharacter: /[^\w\s]/.test(password),
})
