<script lang="ts">
    import { API } from "$lib/api";

    let email = $state("");
    let password = $state("");
    let errorMessage = $state(""); // State to show errors to the user

    async function login(event: SubmitEvent) {
        event.preventDefault();
        errorMessage = ""; // Clear previous errors

        try {
            const response = await fetch(`${API}/login`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email, password })
            });
            
            const resp = await response.json();
            
            if (!response.ok) {
                errorMessage = resp.message || "Invalid email or password.";
                console.log("Login failed");
                return;
            }

            // Check if token actually exists in response
            if (resp.token) {
                localStorage.setItem("token", resp.token);
                window.location.href = "/dashboard";
            } else {
                errorMessage = "Authentication failed. No token received.";
            }

        } catch (error) {
            console.error("Error during login:", error);
            errorMessage = "Something went wrong. Please try again later.";
        }
    }
</script>

<div class="min-h-screen bg-slate-50 flex flex-col justify-center items-center px-4">
  <div class="bg-white p-8 rounded-2xl shadow-xl w-full max-w-md border border-slate-100">
    
    <div class="text-center mb-8">
      <h2 class="text-3xl font-bold text-slate-800 tracking-tight">Sign in</h2>
      <p class="text-slate-500 mt-2 text-sm">Please enter your details to sign in.</p>
    </div>

    {#if errorMessage}
      <div class="mb-4 p-3 bg-red-50 text-red-600 text-sm rounded-xl border border-red-100 font-medium">
        {errorMessage}
      </div>
    {/if}

    <form onsubmit={login} class="space-y-5">
      <div>
        <label for="login-email" class="block text-sm font-semibold text-slate-700 mb-1">Email Address</label>
        <input 
          id="login-email"
          type="email" 
          placeholder="you@example.com" 
          bind:value={email}
          required
          class="w-full px-4 py-2.5 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all" 
        />
      </div>
      
      <div>
        <label for="login-password" class="block text-sm font-semibold text-slate-700 mb-1">Password</label>
        <input 
          id="login-password"
          type="password" 
          placeholder="••••••••" 
          bind:value={password}
          required
          class="w-full px-4 py-2.5 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all" 
        />
      </div>

      <button type="submit" class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-xl shadow-lg shadow-blue-500/20 hover:shadow-blue-500/30 transition-all duration-200 mt-2">
        Sign In
      </button>
    </form>

    <p class="text-center text-sm text-slate-600 mt-6">
      Don't have an account? <a href="/register" class="text-blue-600 hover:underline font-medium">Sign up</a>
    </p>
  </div>
</div>