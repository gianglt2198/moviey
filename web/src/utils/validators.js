/**
 * Validate email format
 * @param {string} email - Email to validate
 * @returns {boolean} True if valid
 */
export const validateEmail = (email) => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
};

/**
 * Validate password strength
 * @param {string} password - Password to validate
 * @returns {Object} Validation result with score and message
 */
export const validatePassword = (password) => {
  let score = 0;
  const feedback = [];

  if (password.length >= 8) score++;
  else feedback.push("At least 8 characters");

  if (/[A-Z]/.test(password)) score++;
  else feedback.push("One uppercase letter");

  if (/[a-z]/.test(password)) score++;
  else feedback.push("One lowercase letter");

  if (/[0-9]/.test(password)) score++;
  else feedback.push("One number");

  if (/[!@#$%^&*]/.test(password)) score++;
  else feedback.push("One special character (!@#$%^&*)");

  return {
    score,
    isValid: score >= 3,
    feedback:
      feedback.length > 0 ? `Add: ${feedback.join(", ")}` : "Strong password",
  };
};

/**
 * Validate required field
 * @param {any} value - Value to validate
 * @returns {boolean} True if not empty
 */
export const isRequired = (value) => {
  return value !== null && value !== undefined && value !== "";
};
