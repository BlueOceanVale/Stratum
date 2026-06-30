<script lang="ts">
    import { API } from "$lib/api";

    let email = $state("");
    let password = $state("");

    async function register(event: SubmitEvent) {
        // Prevent the default browser page reload on form submission
        event.preventDefault();

        try {
            const response = await fetch(`${API}/register`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    email,
                    password,
                })
            });

            if (!response.ok) {
                throw new Error("Registration failed");
            }

            const resp = await response.json();
            console.log(resp);

            // Redirect to login page on success
            window.location.href = "/login";
        } catch (error) {
            console.error("Error during registration:", error);
            // Handle error UI here if needed
        }
    }
</script>

<div class="min-h-screen bg-slate-50 flex flex-col justify-center items-center px-4">
  <div class="bg-white p-8 rounded-2xl shadow-xl w-full max-w-md border border-slate-100">
    
    <div class="text-center mb-8">
      <h2 class="text-3xl font-bold text-slate-800 tracking-tight">Sign up</h2>
      <p class="text-slate-500 mt-2 text-sm">Join us today and get started.</p>
    </div>

    <form onsubmit={register} class="space-y-5">
      <div>
        <label for="email" class="block text-sm font-semibold text-slate-700 mb-1">Email Address</label>
        <input 
          id="email"
          type="email" 
          placeholder="you@example.com" 
          bind:value={email}
          required
          class="w-full px-4 py-2.5 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all" 
        />
      </div>
      
      <div>
        <label for="password" class="block text-sm font-semibold text-slate-700 mb-1">Password</label>
        <input 
          id="password"
          type="password" 
          placeholder="••••••••" 
          bind:value={password}
          required
          class="w-full px-4 py-2.5 border border-slate-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all" 
        />
      </div>

      <button type="submit" class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-xl shadow-lg shadow-blue-500/20 hover:shadow-blue-500/30 transition-all duration-200 mt-2">
        Get Started
      </button>
    </form>

    <p class="text-center text-sm text-slate-600 mt-6">
      Already have an account? <a href="/login" class="text-blue-600 hover:underline font-medium">Log in</a>
    </p>
  </div>
</div>