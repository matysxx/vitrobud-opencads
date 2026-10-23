# Browser updates and offline state

This container serves the app at `/`, not upstream Pages' `/app/`. Relative
manifest, start URL and service-worker registration follow that root. The build
stamps the generated worker cache with a hash of the final bundle, reproducing
upstream Pages packaging without a second version setting. HTTPS remains external.

## Safe update or rollback acceptance

1. Save/export every open drawing to a durable file. Do not rely only on crash
   recovery; browser storage and permissions differ between browser engines.
2. Complete the approved exact-revision server rollout and health checks.
3. Close every tab and installed PWA window for this app. Upstream lets a new
   worker wait until existing tabs close; it never forces reload or skipWaiting
   during an editing session.
4. Reopen through HTTPS online and verify the version and expected commands.
   A clean browser session can distinguish old client state from server failure.
5. If results differ, compare the deployed full SHA/image and served assets,
   then inspect worker/cache state. Save drawings before further action.

Do not routinely clear all site data: it can delete browser-local recovery,
settings and file permissions. Rollback changes server assets, not drawings;
repeat close/reopen acceptance afterward.

Offline use requires a prior successful HTTPS visit and cached assets; it is
not proof of the current server revision. PWA and file APIs may be unavailable
over ordinary HTTP to a LAN address.

## SAVE versus machine export

Ordinary SAVE uses upstream preparation and shared `ocs_save` serialization,
with worker offload or inline fallback. `EXPORTDXFR12` continues to use our
separate ASCII R12 writer and regression contract. Updating upstream does not
prove compatibility with every Fusion/machine importer; real target acceptance
remains a manual check. Browser recovery is not included in host backups.
