<script>
  import { page } from "$app/stores";
</script>

<div class="error-container">
  <div class="error-content">
    <div class="icon">⚠️</div>
    <h1>{$page.status}: {$page.error?.message || "Something went wrong"}</h1>
    <p>
      An unexpected error occurred. Please try again or check the logs if the
      issue persists.
    </p>
    <a href="/" class="btn-primary">Return Home</a>
  </div>
</div>

<style>
  .error-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: #f3f4f6;
    font-family:
      system-ui,
      -apple-system,
      sans-serif;
  }

  .error-content {
    background: white;
    padding: 48px;
    border-radius: 16px;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
    text-align: center;
    max-width: 400px;
  }

  .icon {
    font-size: 64px;
    margin-bottom: 24px;
  }

  h1 {
    margin: 0 0 16px 0;
    color: #1f2937;
    font-size: 24px;
  }

  p {
    margin: 0 0 32px 0;
    color: #6b7280;
    line-height: 1.5;
  }

  .btn-primary {
    display: inline-block;
    background: #2563eb;
    color: white;
    text-decoration: none;
    padding: 12px 24px;
    border-radius: 8px;
    font-weight: 500;
    transition: background 0.2s;
  }

  .btn-primary:hover {
    background: #1d4ed8;
  }
</style>
