import re

with open('website/index.html', 'r') as f:
    html = f.read()

# 1. Update Headings with Gradients
html = html.replace('h1 {', 'h1 {\n      background: linear-gradient(135deg, #FFF4D6, #FFB300);\n      -webkit-background-clip: text;\n      -webkit-text-fill-color: transparent;\n      text-shadow: 0 4px 20px rgba(255, 179, 0, 0.2);')
html = html.replace('h2 {', 'h2 {\n      background: linear-gradient(135deg, #FFF4D6, #FFB300);\n      -webkit-background-clip: text;\n      -webkit-text-fill-color: transparent;\n      border-bottom: none;\n      box-shadow: 0 2px 0 0 rgba(255,255,255,0.05);')

# 2. Add JavaScript at the end of body
js_script = """
  <script>
    document.addEventListener("mousemove", (e) => {
      for(const card of document.getElementsByClassName("glass-panel")) {
        const rect = card.getBoundingClientRect(),
              x = e.clientX - rect.left,
              y = e.clientY - rect.top;
        card.style.setProperty("--mouse-x", `${x}px`);
        card.style.setProperty("--mouse-y", `${y}px`);
      }
    });
  </script>
</body>
"""
html = html.replace('</body>', js_script)

# 3. Update glass-panel CSS
glass_panel_css = """
    @keyframes float {
      0% { transform: translateY(0px); }
      50% { transform: translateY(-10px); }
      100% { transform: translateY(0px); }
    }
    
    .glass-panel {
      position: relative;
      overflow: hidden;
      background: var(--glass-bg);
      backdrop-filter: blur(24px) saturate(180%);
      -webkit-backdrop-filter: blur(24px) saturate(180%);
      border: 1px solid var(--glass-border);
      border-radius: 40px; /* Floating bubble shape */
      box-shadow: 0 12px 40px 0 rgba(0, 0, 0, 0.4), inset 0 0 0 1px rgba(255,255,255,0.05);
      padding: 3rem;
      margin-bottom: 3.5rem;
      animation: float 6s ease-in-out infinite;
      transition: transform 0.3s cubic-bezier(0.25, 0.8, 0.25, 1), box-shadow 0.3s ease;
    }
    
    /* Interactive Liquid Glass Glow on Hover */
    .glass-panel::before {
      content: "";
      position: absolute;
      top: 0; left: 0; right: 0; bottom: 0;
      background: radial-gradient(
        600px circle at var(--mouse-x, 0) var(--mouse-y, 0),
        rgba(255, 215, 0, 0.15),
        transparent 40%
      );
      z-index: -1;
      opacity: 0;
      transition: opacity 0.4s ease;
      pointer-events: none;
    }
    
    .glass-panel:hover {
      transform: translateY(-5px) scale(1.02);
      box-shadow: 0 20px 50px rgba(255, 179, 0, 0.2), inset 0 0 0 1px rgba(255, 179, 0, 0.4);
      animation-play-state: paused;
    }
    
    .glass-panel:hover::before {
      opacity: 1;
    }
"""
# We replace the original `.glass-panel { ... }` block
html = re.sub(r'\.glass-panel\s*\{[^}]+\}', glass_panel_css.strip(), html)

# 4. Inline code styling to be pill bubbles
inline_code_css = """
    code:not(pre code) {
      background: rgba(255, 255, 255, 0.05);
      backdrop-filter: blur(10px);
      border: 1px solid rgba(255, 179, 0, 0.3);
      border-radius: 20px;
      padding: 0.2rem 0.75rem;
      font-family: "JetBrains Mono", monospace;
      color: var(--accent-hover);
      box-shadow: 0 4px 12px rgba(0,0,0,0.2);
      transition: all 0.2s ease;
    }
    code:not(pre code):hover {
      background: rgba(255, 179, 0, 0.15);
      border-color: rgba(255, 215, 0, 0.6);
      box-shadow: 0 6px 15px rgba(255, 179, 0, 0.3);
      transform: translateY(-2px);
    }
"""
html = html.replace('</style>', inline_code_css + '</style>')

with open('website/index.html', 'w') as f:
    f.write(html)
