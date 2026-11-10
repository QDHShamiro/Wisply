(() => {
  if (globalThis.__lt) return;
  const S = { seq: 0, refs: new Map(), ids: new WeakMap(), last: new Set(), muts: 0, lastMuts: -1, lastHref: "", lastY: -1, sig: "", keys: new Map() };
  const OURS = "LT-POINTER";
  const INTERACTIVE =
    'a[href],button,input:not([type="hidden"]),select,textarea,summary,[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="tab"],[role="menuitem"],[role="menuitemcheckbox"],[role="menuitemradio"],[role="option"],[role="combobox"],[role="textbox"],[role="searchbox"],[role="slider"],[role="spinbutton"],[role="treeitem"],[contenteditable=""],[contenteditable="true"],[tabindex]:not([tabindex="-1"]),[onclick]';
  const CONTROL =
    'a[href],button,input:not([type="hidden"]),select,textarea,summary,[role="button"],[role="link"],[role="checkbox"],[role="radio"],[role="switch"],[role="tab"],[role="menuitem"],[role="menuitemcheckbox"],[role="menuitemradio"],[role="option"],[role="combobox"],[role="textbox"],[role="searchbox"],[role="slider"],[role="spinbutton"],[role="treeitem"],[contenteditable=""],[contenteditable="true"]';
  const LANDMARK = 'nav,aside,footer,header,[role="navigation"],[role="complementary"],[role="contentinfo"],[role="banner"]';
  const KEYHINT = /\s*\((?:[a-z0-9] then [a-z0-9]|(?:alt|ctrl|shift|meta|cmd|option|strg|umschalt)(?:[ +-]+(?:alt|ctrl|shift|meta|cmd|option|strg|umschalt|[a-z0-9]))+)\)\s*$/i;
  const KEYHINT1 = /\s*\(([a-z])\)\s*$/;
  const ITEM = 'li,tr,article,[role="listitem"],[role="row"],[role="article"]';
  const ROLE_INPUT = { checkbox: "checkbox", radio: "radio", range: "slider", number: "spinbutton", search: "searchbox", button: "button", submit: "button", reset: "button", image: "button", file: "file", color: "color", date: "date", "datetime-local": "date", time: "time", month: "date", week: "date" };
  const TEXTISH = new Set(["textbox", "searchbox", "spinbutton", "combobox", "date", "time", "color"]);
  const ours = (n) => n.nodeName === OURS;
  new MutationObserver((list) => {
    for (const m of list) {
      const t = m.target;
      if (t && t.nodeType === 1 && (t.tagName === OURS || t.closest(OURS))) continue;
      if (m.type === "childList" && [...m.addedNodes].every(ours) && [...m.removedNodes].every(ours)) continue;
      S.muts++;
      return;
    }
  }).observe(document, { subtree: true, childList: true, attributes: true, characterData: true });

  const clean = (s) => String(s || "").replace(/\s+/g, " ").trim();
  const trunc = (s, n) => (s.length > n ? s.slice(0, n - 3) + "..." : s);
  const q = (s) => s.replace(/"/g, "'");

  function ref(el) {
    let r = S.ids.get(el);
    if (!r) {
      r = "e" + ++S.seq;
      S.ids.set(el, r);
      S.refs.set(r, new WeakRef(el));
    }
    return r;
  }

  function get(r) {
    const w = S.refs.get(r);
    const el = w && w.deref();
    return el && el.isConnected ? el : null;
  }

  function roleOf(el) {
    const r = el.getAttribute("role");
    if (r) return r.split(/\s+/)[0];
    const t = el.tagName;
    if (t === "A") return "link";
    if (t === "BUTTON" || t === "SUMMARY") return "button";
    if (t === "SELECT") return el.multiple ? "listbox" : "combobox";
    if (t === "TEXTAREA") return "textbox";
    if (t === "INPUT") return ROLE_INPUT[(el.type || "text").toLowerCase()] || "textbox";
    if (el.isContentEditable) return "textbox";
    if (/^H[1-6]$/.test(t)) return "heading";
    if (t === "VIDEO") return "video";
    if (t === "AUDIO") return "audio";
    if (t === "IMG") return "img";
    return "clickable";
  }

  const GENERIC_HINT = /^(tux-web-)?(icon-?)?(button|btn|icon|wrapper|container|item|link)$/i;

  function hintOf(el) {
    for (const a of ["data-e2e", "data-testid", "data-test", "data-qa", "data-cy", "id", "name"]) {
      const v = el.getAttribute(a);
      if (v && v.length <= 40 && /[a-z]{3}/i.test(v) && !GENERIC_HINT.test(v) && !/\d{4,}|[a-f0-9]{10,}|^:r/i.test(v)) return v;
    }
    return "";
  }

  function clock(s) {
    if (!isFinite(s)) return "live";
    s = Math.max(0, Math.floor(s));
    const h = Math.floor(s / 3600), m = Math.floor((s % 3600) / 60), sec = String(s % 60).padStart(2, "0");
    return h ? h + ":" + String(m).padStart(2, "0") + ":" + sec : m + ":" + sec;
  }

  function mediaState(el) {
    const d = el.duration;
    let s = " " + clock(el.currentTime) + (isNaN(d) ? "" : "/" + clock(d)) + (el.paused ? " paused" : " playing");
    if (el.muted || el.volume === 0) s += " muted";
    return s;
  }

  function nameOf(el) {
    let s = el.getAttribute("aria-label");
    if (!s) {
      const ids = el.getAttribute("aria-labelledby");
      if (ids) {
        s = ids.split(/\s+/).map((i) => {
          const e = el.ownerDocument.getElementById(i);
          return e ? e.innerText || e.textContent : "";
        }).join(" ");
      }
    }
    const t = el.tagName;
    if (!s && el.labels && el.labels.length) s = [...el.labels].map((l) => l.innerText).join(" ");
    if (!s && (t === "INPUT" || t === "TEXTAREA" || t === "SELECT")) {
      const ty = (el.type || "").toLowerCase();
      s = el.getAttribute("placeholder") || el.getAttribute("title") || (ty === "submit" || ty === "button" || ty === "reset" ? el.value : "") || el.getAttribute("name") || "";
    }
    if (!s && t === "IMG") {
      const cap = el.closest("figure") && el.closest("figure").querySelector("figcaption");
      s = el.alt || (cap ? cap.innerText : "") || el.title;
    }
    if (!s && t !== "INPUT" && t !== "TEXTAREA" && t !== "SELECT") s = el.innerText;
    if (!clean(s)) {
      const inner = el.querySelector("img[alt],[aria-label],[title]");
      if (inner) s = inner.getAttribute("alt") || inner.getAttribute("aria-label") || inner.getAttribute("title");
    }
    if (!clean(s)) s = el.getAttribute("title") || "";
    if (!clean(s) && t === "A" && !el.children.length && el.parentElement) {
      const p = el.parentElement;
      const titled = p.querySelector('h1,h2,h3,h4,[class*="title" i],[class*="name" i]');
      s = titled ? titled.innerText : (p.innerText || "").trim().split("\n")[0];
    }
    const full = clean(s);
    const name = full.replace(KEYHINT, "").replace(KEYHINT1, "");
    if (name && S.keys.size < 40) {
      const hint = KEYHINT.exec(full) || KEYHINT1.exec(full);
      const ks = el.getAttribute("aria-keyshortcuts") || (el.getAttribute("accesskey") ? "alt+" + el.getAttribute("accesskey") : "");
      if (hint) S.keys.set(hint[0].trim().replace(/^\(|\)$/g, ""), name);
      if (ks) S.keys.set(ks, name);
    }
    return name;
  }

  function short(href) {
    try {
      const u = new URL(href, location.href);
      if (u.protocol === "javascript:") return "";
      const yt = /(^|\.)youtube\.com$/.test(u.hostname);
      for (const k of [...u.searchParams.keys()]) {
        if (/^(utm_|fbclid|gclid|dclid|msclkid|yclid|twclid|ttclid|mc_|igshid|_hsenc|_hsmi|mkt_tok)/i.test(k) || (yt && /^(pp|si|feature)$/.test(k))) u.searchParams.delete(k);
        else if (/^((access|id|refresh|auth)_?token|token|code|sig|signature|key|api_?key|secret|password|pass|otp|x-amz-(signature|credential|security-token)|x-goog-(signature|credential))$/i.test(k)) u.searchParams.set(k, "***");
      }
      const qs = u.search.length > 40 ? "?…" : u.search;
      const hex = (s) => (s.includes("/file=") ? s : s.replace(/[0-9a-f]{12,}/g, (m) => m.slice(0, 7) + "…"));
      if (u.origin === location.origin) {
        if (u.pathname === location.pathname && u.search === location.search) return u.hash || ".";
        const here = location.pathname.replace(/\/$/, "");
        const p = here && u.pathname.startsWith(here + "/") ? "." + u.pathname.slice(here.length) : u.pathname;
        return p.includes("/file=") ? p : trunc(hex(p + qs + (u.hash.length > 30 ? "" : u.hash)), 60);
      }
      return trunc(hex(u.host.replace(/^www\./, "") + u.pathname.replace(/\/$/, "") + qs), 60);
    } catch (e) {
      return "";
    }
  }

  function state(el, role, name) {
    let s = "";
    if (role === "checkbox" || role === "radio" || role === "switch" || role === "menuitemcheckbox" || role === "menuitemradio") {
      const a = el.getAttribute("aria-checked");
      const on = el.checked === true || a === "true";
      s += a === "mixed" ? " [-]" : on ? " [x]" : " [ ]";
    } else if (el.hasAttribute("aria-pressed")) {
      const p = el.getAttribute("aria-pressed");
      s += p === "mixed" ? " [-]" : p === "true" ? " [x]" : " [ ]";
    } else if (TEXTISH.has(role)) {
      if (el.tagName === "SELECT") {
        const o = el.selectedOptions && el.selectedOptions[0];
        if (o) s += ' ="' + q(trunc(clean(o.text), 40)) + '"';
      } else if ((el.type || "").toLowerCase() === "password") {
        if (el.value) s += " =***";
      } else if (typeof el.value === "string" && el.value) {
        s += ' ="' + q(trunc(clean(el.value), 40)) + '"';
      }
    }
    if (role === "video" || role === "audio") return mediaState(el);
    if (role === "img") return el.naturalWidth ? " " + el.naturalWidth + "x" + el.naturalHeight : "";
    if (el.disabled || el.getAttribute("aria-disabled") === "true") s += " (disabled)";
    if (el.getAttribute("aria-selected") === "true" || (el.getAttribute("aria-current") && el.getAttribute("aria-current") !== "false")) s += " *";
    const ex = el.getAttribute("aria-expanded");
    if (ex === "true") s += " (expanded)";
    else if (ex === "false") s += " (collapsed)";
    if (role === "link" && el.href) {
      const h = short(el.href);
      if (h && (S.links || !obvious(name, h))) s += " ->" + h;
    }
    return s;
  }

  function alnum(s) {
    return String(s || "").toLowerCase().normalize("NFKD").replace(/[^a-z0-9]/g, "");
  }

  function obvious(name, t) {
    if (t === "." || t === "/") return true;
    const nm = alnum(name);
    if (!nm) return false;
    if (t.startsWith("#")) return alnum(t).length >= 3 && nm.includes(alnum(t).slice(0, 4));
    if (!t.startsWith("/") && !t.startsWith(".")) return false;
    if (t.endsWith("?…")) return true;
    const [path, query = ""] = t.replace(/…$/, "").split("?");
    let seg = path.split("#")[0].split("/").filter(Boolean).pop() || "";
    try {
      seg = decodeURIComponent(seg);
    } catch (e) {}
    seg = alnum(seg);
    if (query && alnum(query).includes(nm.slice(0, 6))) return true;
    if (/^\d+$/.test(seg)) return nm.includes(seg);
    if (seg.length < 3) return false;
    let k = 0;
    while (k < seg.length && k < nm.length && seg[k] === nm[k]) k++;
    return nm.includes(seg) || seg.includes(nm) || k >= 4;
  }

  function line(el, role, name) {
    const hint = name || role === "video" || role === "audio" || role === "img" ? "" : hintOf(el);
    return ref(el) + " " + role + (name ? ' "' + q(trunc(name, 60)) + '"' : hint ? " #" + hint : "") + state(el, role, name);
  }

  function frameOffset(doc) {
    let x = 0, y = 0, win = doc.defaultView;
    while (win && win !== window && win.frameElement) {
      const fe = win.frameElement;
      const r = fe.getBoundingClientRect();
      x += r.left + fe.clientLeft;
      y += r.top + fe.clientTop;
      win = win.parent;
    }
    return { x, y };
  }

  function covered(el, r) {
    const root = el.getRootNode();
    const pts = [[r.left + r.width / 2, r.top + r.height / 2], [r.left + r.width * 0.25, r.top + r.height / 2], [r.left + r.width * 0.75, r.top + r.height / 2]];
    let hit = null;
    for (const [x, y] of pts) {
      hit = root.elementFromPoint ? root.elementFromPoint(x, y) : null;
      if (!hit || hit === el || el.contains(hit) || hit.tagName === OURS) return null;
      if (hit.shadowRoot && hit.shadowRoot.contains(el)) return null;
    }
    return hit;
  }

  function describe(el) {
    if (!el) return "?";
    const cls = typeof el.className === "string" ? el.className.trim().split(/\s+/).slice(0, 2).join(".") : "";
    const nm = trunc(nameOf(el), 30);
    return el.tagName.toLowerCase() + (el.id ? "#" + el.id : "") + (cls ? "." + cls : "") + (nm ? ' "' + q(nm) + '"' : "");
  }

  function pointerish(el, cs) {
    if (cs.cursor !== "pointer") return false;
    const p = el.parentElement;
    return !p || getComputedStyle(p).cursor !== "pointer";
  }

  function where(ax, ay, w, h) {
    if (ax + w <= 0 || ax >= innerWidth) return "away";
    if (ay >= innerHeight) return "below";
    if (ay + h <= 0) return scrollY > 0 && ay + scrollY + h > 0 ? "above" : "away";
    return "in";
  }

  function offscreen(n, off, cut) {
    const r = n.getBoundingClientRect();
    if (!r.width && !r.height) return false;
    const pos = where(r.left + off.x, r.top + off.y, r.width, r.height);
    if (pos === "in") return false;
    if (pos !== "away") cut[pos] += n.querySelectorAll(INTERACTIVE).length;
    return true;
  }

  function walk(root, off, out, frames, cut) {
    const doc = root.ownerDocument || root;
    const tw = doc.createTreeWalker(root, NodeFilter.SHOW_ELEMENT, {
      acceptNode(n) {
        const t = n.tagName;
        if (t === "SCRIPT" || t === "STYLE" || t === "NOSCRIPT" || t === "TEMPLATE" || t === OURS || t === "HEAD") return NodeFilter.FILTER_REJECT;
        if (!n.checkVisibility()) {
          return getComputedStyle(n).display === "contents" ? NodeFilter.FILTER_SKIP : NodeFilter.FILTER_REJECT;
        }
        if (cut && cut.prune && n.matches(ITEM) && offscreen(n, off, cut)) return NodeFilter.FILTER_REJECT;
        return NodeFilter.FILTER_ACCEPT;
      },
    });
    let n = tw.nextNode();
    while (n) {
      if (cut && cut.until && (cut.late || performance.now() > cut.until)) {
        cut.late = true;
        return;
      }
      const t = n.tagName;
      const cs = t === "BODY" || t === "HTML" ? null : getComputedStyle(n);
      let isInt = n.matches(INTERACTIVE);
      if (isInt && !n.matches(CONTROL)) {
        isInt = !n.querySelector(INTERACTIVE) && (n.hasAttribute("onclick") || (cs && cs.cursor === "pointer"));
      }
      if (cut && cs && (cs.position === "fixed" || cs.position === "sticky" || cs.position === "absolute")) cut.layers.push({ el: n, off });
      const head = !isInt && (/^H[1-3]$/.test(t) || n.getAttribute("role") === "heading");
      const media = (t === "VIDEO" || (t === "AUDIO" && n.controls) || (t === "IMG" && cut && cut.images)) && !isInt;
      if (isInt || head || media || (cs && pointerish(n, cs) && !n.closest(CONTROL) && !n.querySelector(INTERACTIVE))) {
        out.push({ el: n, off, heading: head, media });
      }
      if (n.shadowRoot) walk(n.shadowRoot, off, out, frames, cut);
      if (t === "IFRAME" || t === "FRAME") {
        let inner = null;
        try {
          inner = n.contentDocument;
        } catch (e) {}
        if (inner && inner.documentElement) {
          const r = n.getBoundingClientRect();
          walk(inner.documentElement, { x: off.x + r.left + n.clientLeft, y: off.y + r.top + n.clientTop }, out, frames, cut);
        } else {
          frames.push(n);
        }
      }
      n = tw.nextNode();
    }
  }

  function flushAnimations() {
    if (!document.getAnimations) return;
    for (const a of document.getAnimations()) {
      try {
        if (a.playState === "running" && a.effect && a.effect.getComputedTiming().endTime !== Infinity) a.finish();
      } catch (e) {}
    }
  }

  function rect(r, opts) {
    const el = get(r);
    if (!el) return { gone: true };
    let b = el.getBoundingClientRect();
    const off = frameOffset(el.ownerDocument);
    const ax = b.left + off.x, ay = b.top + off.y;
    if (!opts || !opts.noScroll) {
      if (ay < 0 || ax < 0 || ay + b.height > innerHeight || ax + b.width > innerWidth) {
        el.scrollIntoView({ block: "center", inline: "center", behavior: "instant" });
        b = el.getBoundingClientRect();
      }
    }
    const o2 = frameOffset(el.ownerDocument);
    const hit = covered(el, b);
    return {
      x: Math.round(b.left + o2.x + b.width / 2),
      y: Math.round(b.top + o2.y + b.height / 2),
      w: Math.round(b.width),
      h: Math.round(b.height),
      name: nameOf(el),
      role: roleOf(el),
      media: el.tagName === "VIDEO" || el.tagName === "AUDIO" ? mediaState(el).trim() : "",
      covered: hit ? describe(hit) : null,
      dpr: devicePixelRatio,
      href: el.tagName === "A" ? el.href : "",
      blank: el.tagName === "A" && el.target === "_blank",
      line: line(el, roleOf(el), nameOf(el)),
    };
  }

  function hay(el) {
    const text = el.textContent || "";
    let h = text;
    for (const a of ["aria-label", "title", "placeholder", "alt", "name"]) {
      const v = el.getAttribute(a);
      if (v) h += " " + v;
    }
    if (typeof el.value === "string") h += " " + el.value;
    const lb = el.getAttribute("aria-labelledby");
    if (lb) for (const id of lb.split(/\s+/)) h += " " + ((el.ownerDocument.getElementById(id) || {}).textContent || "");
    if (el.labels) for (const l of el.labels) h += " " + l.textContent;
    const inner = text.trim() ? null : el.querySelector("img[alt],[aria-label],[title]");
    if (inner) h += " " + (inner.getAttribute("alt") || "") + (inner.getAttribute("aria-label") || "") + (inner.getAttribute("title") || "");
    return h.toLowerCase().replace(/\s+/g, "");
  }

  function candidates(root, needle, viewOnly) {
    flushAnimations();
    const items = [], frames = [];
    walk(root || document.documentElement, { x: 0, y: 0 }, items, frames, viewOnly ? { below: 0, above: 0, layers: [], prune: true } : undefined);
    const vw = innerWidth, vh = innerHeight, out = [];
    for (const it of items) {
      if (it.heading) continue;
      const el = it.el;
      if (needle && !hay(el).includes(needle)) continue;
      const r = el.getBoundingClientRect();
      if (r.width < 1 || r.height < 1) continue;
      if (!el.checkVisibility({ opacityProperty: true, visibilityProperty: true })) continue;
      if (el.closest('[aria-hidden="true"]')) continue;
      const ax = r.left + it.off.x, ay = r.top + it.off.y;
      const inView = ay + r.height > 0 && ax + r.width > 0 && ay < vh && ax < vw;
      if (viewOnly && !inView) continue;
      out.push({ el, role: roleOf(el), name: nameOf(el), inView });
    }
    return out;
  }

  globalThis.__lt = { S, get, ref, roleOf, nameOf, hintOf, clean, trunc, q, describe, covered, frameOffset, rect, candidates, line, INTERACTIVE, CONTROL, ITEM, LANDMARK, mediaState, where, walk, flushAnimations };
})();
