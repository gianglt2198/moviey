import React from "react";
import "./Button.css";

/**
 * Reusable Button Component
 * @param {string} variant - Button style (primary, secondary, danger, ghost)
 * @param {string} size - Button size (sm, md, lg)
 * @param {string} icon - Icon emoji or text
 * @param {boolean} loading - Show loading state
 * @param {boolean} disabled - Disable button
 * @param {function} onClick - Click handler
 * @param {ReactNode} children - Button text
 */
export const Button = ({
  variant = "primary",
  size = "md",
  icon,
  loading = false,
  disabled = false,
  onClick,
  className = "",
  fullWidth = false,
  ...props
}) => {
  const buttonClass = [
    "btn",
    `btn-${variant}`,
    `btn-${size}`,
    fullWidth && "btn-full-width",
    loading && "btn-loading",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <button
      className={buttonClass}
      disabled={disabled || loading}
      onClick={onClick}
      {...props}
    >
      {loading && <span className="btn-loader" />}
      {icon && <span className="btn-icon">{icon}</span>}
      <span>{props.children}</span>
    </button>
  );
};
