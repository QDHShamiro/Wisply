(() => {
  const L = globalThis.__lt;
  if (!L || L.media) return;
  const { S, get, ref, clean, mediaState } = L;

  function captionAt(el, t) {
    const parts = [];
    for (const x of el.textTracks || []) {
      if (x.mode === "disabled" || !x.cues) continue;
      for (const c of x.cues) if (c.startTime <= t && t < c.endTime) parts.push(c.text);
    }
    if (!parts.length) for (const d of document.querySelectorAll(".ytp-caption-segment")) parts.push(d.innerText);
    return clean(parts.join(" ").replace(/<[^>]+>/g, " "));
  }

  const painted = () => new Promise((res) => {
    const t = setTimeout(res, 150);
    requestAnimationFrame(() => requestAnimationFrame(() => {
      clearTimeout(t);
      res();
    }));
  });

  async function media(r, op, t) {
    const el = get(r);
    if (!el) return { gone: true };
    if (el.tagName !== "VIDEO" && el.tagName !== "AUDIO") return { notMedia: true };
    if (op === "mark" || op === "unmark") {
      if (op === "mark") el.setAttribute("data-lt-play", "1");
      else el.removeAttribute("data-lt-play");
      return { paused: el.paused };
    }
    if (op === "pause") {
      if (!el.paused) el.pause();
      return { paused: true };
    }
    if (op === "info") {
      S.media = { el, time: el.currentTime, paused: el.paused || el.readyState < 2, controls: el.controls, tracks: [...el.textTracks].map((x) => x.mode) };
      el.controls = false;
      for (const x of el.textTracks) if (x.mode === "disabled" && (x.kind === "subtitles" || x.kind === "captions")) x.mode = "hidden";
      const d = el.duration;
      return { duration: isFinite(d) ? d : 0, time: el.currentTime, paused: el.paused, seekable: isFinite(d) && d > 1 && el.seekable.length > 0 };
    }
    if (op === "wait") {
      const t0 = Date.now();
      const loading = () => el.networkState === el.NETWORK_LOADING || Date.now() - t0 < 300;
      while (el.readyState < 1 && Date.now() - t0 < 4000 && loading()) await new Promise((res) => setTimeout(res, 100));
      const d = el.duration;
      return { duration: isFinite(d) ? d : 0, seekable: isFinite(d) && d > 1 && el.seekable.length > 0 };
    }
    if (op === "prime" && el.played.length === 0) {
      const at = el.currentTime;
      el.play().catch(() => {});
      const t0 = Date.now();
      while (el.currentTime === at && Date.now() - t0 < 3000) await new Promise((res) => setTimeout(res, 100));
      el.pause();
    }
    if (op === "seek") {
      if (!el.paused) el.pause();
      if (Math.abs(el.currentTime - t) > 0.05) {
        await new Promise((res) => {
          const done = () => {
            el.removeEventListener("seeked", done);
            res();
          };
          el.addEventListener("seeked", done);
          setTimeout(done, 4000);
          el.currentTime = t;
        });
      }
      const t0 = Date.now();
      while (el.readyState < 2 && Date.now() - t0 < 3000) await new Promise((res) => setTimeout(res, 50));
      await painted();
    }
    if (op === "cues") {
      for (const x of el.textTracks) if (x.mode === "disabled" && (x.kind === "subtitles" || x.kind === "captions")) x.mode = "hidden";
      const t0 = Date.now();
      let track = null;
      while (!track && Date.now() - t0 < 1500) {
        track = [...el.textTracks].find((x) => x.cues && x.cues.length);
        if (!track) await new Promise((res) => setTimeout(res, 100));
      }
      return { cues: track ? [...track.cues].map((c) => [c.startTime, clean(String(c.text || "").replace(/<[^>]+>/g, " "))]) : [], lang: track ? track.language : "", duration: isFinite(el.duration) ? el.duration : 0 };
    }
    if (op === "record") {
      const cap = el.captureStream ? el.captureStream() : null;
      const tracks = cap ? cap.getAudioTracks() : [];
      if (!tracks.length) return { error: "no audio track to record (no sound, or the site blocks capturing)" };
      const rec = new MediaRecorder(new MediaStream(tracks), { mimeType: "audio/webm;codecs=opus", audioBitsPerSecond: 32000 });
      const chunks = [];
      rec.ondataavailable = (e) => {
        if (e.data && e.data.size) chunks.push(e.data);
      };
      const was = { muted: el.muted, volume: el.volume, paused: el.paused };
      el.muted = false;
      if (el.volume === 0) el.volume = 1;
      const start = el.currentTime;
      rec.start(1000);
      let blocked = false;
      if (el.paused) await el.play().catch(() => (blocked = true));
      if (blocked) {
        el.muted = true;
        await el.play().catch(() => {});
      }
      const t0 = Date.now();
      while (Date.now() - t0 < t && !el.ended && !el.paused) await new Promise((res) => setTimeout(res, 200));
      await new Promise((res) => {
        rec.onstop = res;
        rec.stop();
      });
      el.muted = was.muted;
      el.volume = was.volume;
      if (was.paused && !el.paused) el.pause();
      const buf = new Uint8Array(await new Blob(chunks, { type: "audio/webm" }).arrayBuffer());
      let bin = "";
      for (let i = 0; i < buf.length; i += 32768) bin += String.fromCharCode.apply(null, buf.subarray(i, i + 32768));
      return { audio: btoa(bin), start, seconds: (Date.now() - t0) / 1000, blocked };
    }
    if (op === "restore" && S.media && S.media.el === el) {
      const m = S.media;
      S.media = null;
      if (t !== -1) el.currentTime = m.time;
      el.controls = m.controls;
      [...el.textTracks].forEach((x, i) => {
        if (m.tracks[i] !== undefined) x.mode = m.tracks[i];
      });
      if (m.paused && !el.paused) el.pause();
      if (!m.paused && el.paused) el.play().catch(() => {});
    }
    return { media: mediaState(el).trim(), cap: captionAt(el, el.currentTime) };
  }

  function firstMedia() {
    const all = [...document.querySelectorAll("video,audio")];
    const el = all.find((e) => !e.paused) || all.find((e) => e.readyState > 0) || all[0];
    return el ? { ref: ref(el), duration: isFinite(el.duration) ? el.duration : 0 } : { none: true };
  }

  Object.assign(L, { media, firstMedia });
})();
