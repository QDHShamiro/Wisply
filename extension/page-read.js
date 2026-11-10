(() => {
  const L = globalThis.__lt;
  if (!L || L.read) return;
  const { S, get, ref, clean } = L;

  const visible = (el) => {
    if (!el || !el.checkVisibility || !el.checkVisibility({ opacityProperty: true, visibilityProperty: true })) return false;
    const r = el.getBoundingClientRect();
    return r.width > 0 && r.height > 0;
  };

  function hiddenFree() {
    const marked = [];
    const tw = document.createTreeWalker(document.body || document.documentElement, NodeFilter.SHOW_ELEMENT, {
      acceptNode(n) {
        const gone = n.tagName === "LT-POINTER" || n.getAttribute("aria-hidden") === "true" || !n.checkVisibility({ opacityProperty: true, visibilityProperty: true });
        if (!gone) return NodeFilter.FILTER_ACCEPT;
        if (getComputedStyle(n).display === "contents") return NodeFilter.FILTER_SKIP;
        n.setAttribute("data-lt-hidden", "");
        marked.push(n);
        return NodeFilter.FILTER_REJECT;
      },
    });
    while (tw.nextNode()) {}
    const clone = document.documentElement.cloneNode(true);
    for (const n of marked) n.removeAttribute("data-lt-hidden");
    for (const n of clone.querySelectorAll("[data-lt-hidden],script,style,noscript,template")) n.remove();
    const doc = document.implementation.createHTMLDocument(document.title);
    doc.replaceChild(doc.importNode(clone, true), doc.documentElement);
    return doc;
  }

  function text(opts) {
    opts = opts || {};
    let md = "", title = document.title, note = "";
    if (!globalThis.Defuddle && !opts.raw) note = "readable view not loaded" + (opts.why ? ": " + opts.why : "");
    if (globalThis.Defuddle && !opts.raw) {
      try {
        const res = new globalThis.Defuddle(hiddenFree(), { markdown: true, url: location.href }).parse();
        md = res.content || "";
        title = res.title || title;
        if (!md.trim()) note = "readable view was empty";
      } catch (e) {
        md = "";
        note = "readable view failed: " + String((e && e.message) || e).slice(0, 120);
      }
    }
    if (!opts.links) {
      md = md.split(/(```[\s\S]*?```|`[^`\n]*`)/).map((p, i) => (i % 2 ? p : p.replace(/!?\[([^\]]*)\]\((?:[^()]|\([^()]*\))*\)/g, "$1").replace(/\[\^[^\]]+\](?!:)/g, "").replace(/<(iframe|video|audio|embed|object|svg)\b[^>]*>(?:[\s\S]*?<\/\1>)?/gi, ""))).join("");
    }
    const main = document.querySelector('main,[role="main"],article') || document.body || document.documentElement;
    if (md.trim() && md.length < 3000) {
      const plain = main.innerText || "";
      if (plain.length > 1500 && md.length < plain.length * 0.3) md = "";
    }
    if (!md.trim()) {
      md = main.innerText || "";
      if (note) md = "(" + note + ", plain text follows)\n" + md;
    }
    md = md.replace(/[ \t]+\n/g, "\n").replace(/\n{3,}/g, "\n\n").trim();
    if (opts.filter) {
      const fl = opts.filter.toLowerCase().split("|").map((s) => s.trim()).filter(Boolean);
      const parts = md.split(/\n\n+/);
      const keep = [];
      parts.forEach((p, i) => {
        const low = p.toLowerCase();
        if (fl.some((f) => low.includes(f))) {
          if (i > 0 && /^#/.test(parts[i - 1]) && !keep.includes(parts[i - 1])) keep.push(parts[i - 1]);
          keep.push(p);
        }
      });
      md = keep.join("\n\n") || "(no paragraph contains '" + opts.filter + "')";
    }
    return { title, url: location.href, text: md };
  }

  function table(target) {
    let el = null;
    if (target) el = /^e\d+$/.test(target) ? get(target) : document.querySelector(target);
    if (el && el.tagName !== "TABLE" && el.getAttribute("role") !== "grid" && el.getAttribute("role") !== "table") {
      el = el.closest('table,[role="grid"],[role="table"]') || el.querySelector('table,[role="grid"],[role="table"]');
    }
    if (!el) el = [...document.querySelectorAll('table,[role="grid"],[role="table"]')].find(visible);
    if (!el) return { none: true };
    const rows = el.tagName === "TABLE" ? [...el.rows] : [...el.querySelectorAll('[role="row"]')];
    const out = rows.map((r) => {
      const cells = el.tagName === "TABLE" ? [...r.cells] : [...r.querySelectorAll('[role="cell"],[role="gridcell"],[role="columnheader"],[role="rowheader"]')];
      return cells.map((c) => clean(c.innerText).replace(/\t/g, " ")).join("\t");
    }).filter((r) => r.trim());
    return { ref: ref(el), rows: out };
  }

  function viewSize() {
    return { w: innerWidth, h: innerHeight };
  }

  function frameBox(url, size, idx) {
    const base = (u) => String(u || "").split("#")[0];
    const origin = (u) => {
      try {
        return new URL(u, location.href).origin;
      } catch (e) {
        return "";
      }
    };
    const all = [...document.querySelectorAll("iframe,frame")];
    let c = all.filter((f) => base(f.src) === base(url));
    if (!c.length) c = all.filter((f) => origin(f.src) === origin(url));
    const fit = c.filter((f) => Math.abs(f.clientWidth - size.w) < 3 && Math.abs(f.clientHeight - size.h) < 3);
    if (fit.length) c = fit;
    const f = c[Math.min(Math.max(idx, 0), c.length - 1)];
    if (!f) return null;
    let r = f.getBoundingClientRect();
    if (r.bottom < 1 || r.right < 1 || r.top > innerHeight - 1 || r.left > innerWidth - 1) {
      f.scrollIntoView({ block: "center", inline: "center", behavior: "instant" });
      r = f.getBoundingClientRect();
    }
    const cs = getComputedStyle(f);
    return { x: r.left + f.clientLeft + (parseFloat(cs.paddingLeft) || 0), y: r.top + f.clientTop + (parseFloat(cs.paddingTop) || 0) };
  }

  function settle(quiet, max, waitFirst, loaded) {
    if (loaded && document.readyState === "complete") quiet = Math.min(quiet, loaded);
    return new Promise((resolve) => {
      const t0 = Date.now(), start = S.muts;
      let last = S.muts, changed = t0, seen = false;
      const iv = setInterval(() => {
        const now = Date.now();
        if (S.muts !== last) {
          last = S.muts;
          changed = now;
          seen = true;
        }
        if (waitFirst && !seen) {
          if (now - t0 >= waitFirst) {
            clearInterval(iv);
            resolve({ ms: now - t0, changed: false });
          }
          return;
        }
        if (now - changed >= quiet || now - t0 >= max) {
          clearInterval(iv);
          resolve({ ms: now - t0, changed: S.muts !== start });
        }
      }, 25);
    });
  }

  function scrollInfo() {
    return { y: Math.round(scrollY), h: document.documentElement.scrollHeight, vh: innerHeight, vw: innerWidth };
  }

  function roomIn(el, dy) {
    if (el.scrollHeight <= el.clientHeight + 1) return 0;
    return dy < 0 ? el.scrollTop : el.scrollHeight - el.clientHeight - el.scrollTop;
  }

  function scroller(x, y, dy) {
    let el = document.elementFromPoint(x, y);
    while (el && el !== document.body && el !== document.documentElement) {
      if (/(auto|scroll|overlay)/.test(getComputedStyle(el).overflowY) && roomIn(el, dy) > 1) return el;
      el = el.parentElement || (el.getRootNode() && el.getRootNode().host) || null;
    }
    return document.scrollingElement || document.documentElement;
  }

  function scrollBy(x, y, dy, to) {
    const el = scroller(x, y, to === "top" ? -1 : to === "bottom" ? 1 : dy);
    const before = el.scrollTop;
    if (to === "top") el.scrollTo({ top: 0, behavior: "instant" });
    else if (to === "bottom") el.scrollTo({ top: el.scrollHeight, behavior: "instant" });
    else el.scrollBy({ top: dy, behavior: "instant" });
    const max = el.scrollHeight - el.clientHeight;
    return {
      moved: Math.abs(el.scrollTop - before) >= 1,
      scrollable: max > 1,
      pct: max > 1 ? Math.round((el.scrollTop / max) * 100) : 0,
    };
  }

  function visibleText(t) {
    const tl = t.toLowerCase();
    const tw = document.createTreeWalker(document.body || document.documentElement, NodeFilter.SHOW_TEXT);
    let n;
    while ((n = tw.nextNode())) {
      if (!n.data.toLowerCase().includes(tl)) continue;
      const p = n.parentElement;
      if (!p || !visible(p)) continue;
      const r = p.getBoundingClientRect();
      if (r.bottom > 0 && r.top < innerHeight) return { ok: true, ref: ref(p) };
    }
    return { ok: false };
  }

  function peek(max) {
    const tw = document.createTreeWalker(document.body || document.documentElement, NodeFilter.SHOW_TEXT);
    const out = [];
    let n, len = 0;
    while ((n = tw.nextNode()) && len < max) {
      const s = n.data.replace(/\s+/g, " ").trim();
      if (!s) continue;
      const p = n.parentElement;
      if (!p || /^(SCRIPT|STYLE|NOSCRIPT|TEMPLATE)$/.test(p.tagName) || !visible(p)) continue;
      const r = p.getBoundingClientRect();
      if (r.width <= 1 || r.height <= 1 || r.bottom <= 0 || r.top >= innerHeight) continue;
      out.push(s);
      len += s.length + 1;
    }
    const t = out.join(" ");
    return t.length > max ? t.slice(0, max - 3) + "..." : t;
  }

  L.read = { visible, text, table, viewSize, frameBox, settle, scrollInfo, scrollBy, visibleText, peek };
})();
