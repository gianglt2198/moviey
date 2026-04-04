import React from "react";
import "./Card.css";

/**
 * Reusable Card Component
 * @param {ReactNode} children - Card content
 * @param {string} variant - Card style (default, hover, interactive)
 * @param {ReactNode} header - Card header
 * @param {ReactNode} footer - Card footer
 * @param {function} onClick - Click handler
 */
export const Card = ({
  children,
  variant = "default",
  header,
  footer,
  onClick,
  className = "",
  ...props
}) => {
  return (
    <div
      className={`card card-${variant} ${className}`}
      onClick={onClick}
      role={onClick ? "button" : "article"}
      tabIndex={onClick ? 0 : -1}
      {...props}
    >
      {header && <div className="card-header">{header}</div>}
      <div className="card-body">{children}</div>
      {footer && <div className="card-footer">{footer}</div>}
    </div>
  );
};
