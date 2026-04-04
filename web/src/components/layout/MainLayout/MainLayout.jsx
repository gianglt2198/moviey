import React from "react";
import "./MainLayout.css";

/**
 * Main Layout Component
 * Wrapper for app pages with header, main content, and footer
 */
export const MainLayout = ({
  children,
  header,
  footer,
  sidebar,
  className = "",
}) => {
  return (
    <div className={`main-layout ${className}`}>
      {header && <header className="layout-header">{header}</header>}

      <div className="layout-body">
        {sidebar && <aside className="layout-sidebar">{sidebar}</aside>}
        <main className="layout-main">{children}</main>
      </div>

      {footer && <footer className="layout-footer">{footer}</footer>}
    </div>
  );
};
