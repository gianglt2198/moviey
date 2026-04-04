import React, { useState } from "react";
import { useAuthContext } from "../context";
import { validateEmail, validatePassword } from "../utils/validators";
import { getErrorMessage } from "../utils/errorHandler";
import "../styles/pages/LoginPage.css";
import { Input, Button } from "@components";

function LoginPage() {
  const { login, loading, error: authError } = useAuthContext();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleLogin = async (e) => {
    e.preventDefault();
    setError("");

    // Validate inputs
    if (!email || !password) {
      setError("Email and password are required");
      return;
    }

    if (!validateEmail(email)) {
      setError("Please enter a valid email address");
      return;
    }

    try {
      await login(email, password);
      setEmail("");
      setPassword("");
    } catch (err) {
      setError(getErrorMessage(err));
    }
  };

  const displayError = error || authError;

  return (
    <div className="login-container">
      <div className="login-box">
        <h1>🎬 Moviey</h1>
        <p className="tagline">Your Personal Streaming Hub</p>

        <form onSubmit={handleLogin}>
          {displayError && <div className="error-message">{displayError}</div>}

          {/* <input
            type="email"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            disabled={loading}
          /> */}

          <Input
            type="email"
            label="Email"
            placeholder="your@email.com"
            icon="✉️"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            error={error}
            required
          />

          <Input
            type="password"
            label="Password"
            placeholder="Password"
            icon="🔒"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            error={error}
            required
          />

          <Button type="submit" disabled={loading} className="login-btn">
            {loading ? "🔄 Logging in..." : "Login"}
          </Button>
        </form>

        <p className="signup-hint">Demo: test@example.com / TestPass123</p>
      </div>
    </div>
  );
}

export default LoginPage;
