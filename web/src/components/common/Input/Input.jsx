import React, { useState } from "react";
import "./Input.css";

/**
 * Reusable Input Component
 * @param {string} type - Input type
 * @param {string} label - Input label
 * @param {string} placeholder - Input placeholder
 * @param {string} error - Error message
 * @param {boolean} required - Mark as required
 * @param {string} icon - Icon emoji
 */
export const Input = React.forwardRef(
  (
    {
      type = "text",
      label,
      placeholder,
      error,
      required,
      icon,
      disabled = false,
      value,
      onChange,
      onBlur,
      className = "",
      ...props
    },
    ref,
  ) => {
    const [isFocused, setIsFocused] = useState(false);

    return (
      <div className={`input-group ${error ? "error" : ""}`}>
        {label && (
          <label className="input-label">
            {label}
            {required && <span className="required">*</span>}
          </label>
        )}

        <div className="input-wrapper">
          {icon && <span className="input-icon">{icon}</span>}
          <input
            ref={ref}
            type={type}
            placeholder={placeholder}
            disabled={disabled}
            value={value}
            onChange={onChange}
            onBlur={onBlur}
            onFocus={() => setIsFocused(true)}
            onBlurCapture={() => setIsFocused(false)}
            className={`input-field ${icon ? "has-icon" : ""} ${className}`}
            {...props}
          />
        </div>

        {error && <span className="input-error">{error}</span>}
      </div>
    );
  },
);

Input.displayName = "Input";
