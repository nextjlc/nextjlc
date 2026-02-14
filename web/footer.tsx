/* web/footer.tsx */

import { GitCommitVertical } from "lucide-react";

function Footer() {
  return (
    <footer className="bg-(--color-bg-alt) text-(--color-text) px-8 py-4">
      <div className="hidden md:flex items-center justify-between">
        <div className="flex flex-col gap-0">
          <div className="text-(--color-subtext)">
            &copy; 2024-2026{" "}
            <a
              href="https://github.com/nextjlc"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-(--color-text) hover:underline"
            >
              NeXTJLC
            </a>
            .{" "}
            <a
              href="https://opensource.org/licenses/MIT"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-(--color-text) hover:underline"
            >
              Released under the MIT License.
            </a>
          </div>
          <div className="text-(--color-subtext)">
            <span>Thank for </span>
            <a
              href="https://github.com/acha666/FuckJLC/tree/main"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-(--color-text) hover:underline"
            >
              FuckJLC
            </a>{" "}
            <a
              href="https://github.com/HalfSweet/TransJLC"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-(--color-text) hover:underline"
            >
              TransJLC
            </a>{" "}
            <a
              href="https://github.com/nextjlc/openjlc"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-(--color-text) hover:underline"
            >
              OpenJLC
            </a>
          </div>
        </div>
        <div className="text-sm flex flex-col items-end">
          <div className="flex items-center text-(--color-subtext)">
            <GitCommitVertical className="h-4 w-4 mr-1" />
            <span>Commit-</span>
            <a
              href={`https://github.com/nextjlc/nextjlc/tree/${__GIT_HASH__}`}
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-(--color-text) hover:underline"
            >
              {__GIT_HASH__}
            </a>
          </div>
          <a
            href="https://github.com/nextjlc/nextjlc"
            target="_blank"
            rel="noopener noreferrer"
            className="text-(--color-subtext) hover:text-(--color-text) hover:underline mt-1"
          >
            View Source on GitHub
          </a>
        </div>
      </div>
      <div className="md:hidden flex flex-col items-center text-center gap-1">
        <div className="text-(--color-subtext) text-sm">
          &copy; 2025{" "}
          <a
            href="https://github.com/nextjlc"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-(--color-text) hover:underline"
          >
            NeXTJLC
          </a>
          .{" "}
          <a
            href="https://opensource.org/licenses/MIT"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-(--color-text) hover:underline"
          >
            Released under the MIT License.
          </a>
        </div>
        <div className="text-(--color-subtext) text-sm">
          <span>Thank for </span>
          <a
            href="https://github.com/acha666/FuckJLC/tree/main"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-(--color-text) hover:underline"
          >
            FuckJLC
          </a>{" "}
          <a
            href="https://github.com/HalfSweet/TransJLC"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-(--color-text) hover:underline"
          >
            TransJLC
          </a>{" "}
          <a
            href="https://github.com/nextjlc/openjlc"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-(--color-text) hover:underline"
          >
            OpenJLC
          </a>
        </div>
      </div>
    </footer>
  );
}

export default Footer;
