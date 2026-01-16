/* web/footer.tsx */

import { GitCommitVertical } from "lucide-react";

function Footer() {
  return (
    <footer className="bg-[var(--blue-color)] text-gray-200 px-8 py-4">
      <div className="hidden md:flex items-center justify-between">
        <div className="flex flex-col gap-0">
          <div className="text-gray-400">
            &copy; 2024-2026{" "}
            <a
              href="https://github.com/nextjlc"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-white hover:underline"
            >
              NeXTJLC
            </a>
            .{" "}
            <a
              href="https://opensource.org/licenses/MIT"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-white hover:underline"
            >
              Released under the MIT License.
            </a>
          </div>
          <div className="text-gray-400">
            <span>Thank for </span>
            <a
              href="https://github.com/acha666/FuckJLC/tree/main"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-white hover:underline"
            >
              FuckJLC
            </a>{" "}
            <a
              href="https://github.com/HalfSweet/TransJLC"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-white hover:underline"
            >
              TransJLC
            </a>{" "}
            <a
              href="https://github.com/nextjlc/openjlc"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-white hover:underline"
            >
              OpenJLC
            </a>
          </div>
        </div>
        <div className="text-sm flex flex-col items-end">
          <div className="flex items-center text-gray-400">
            <GitCommitVertical className="h-4 w-4 mr-1" />
            <span>Commit-</span>
            <a
              href={`https://github.com/nextjlc/nextjlc/tree/${__GIT_HASH__}`}
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-white hover:underline"
            >
              {__GIT_HASH__}
            </a>
          </div>
          <a
            href="https://github.com/nextjlc/core"
            target="_blank"
            rel="noopener noreferrer"
            className="text-gray-400 hover:text-white hover:underline mt-1"
          >
            View Source on GitHub
          </a>
        </div>
      </div>
      <div className="md:hidden flex flex-col items-center text-center gap-1">
        <div className="text-gray-400 text-sm">
          &copy; 2025{" "}
          <a
            href="https://github.com/nextjlc"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-white hover:underline"
          >
            NeXTJLC
          </a>
          .{" "}
          <a
            href="https://opensource.org/licenses/MIT"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-white hover:underline"
          >
            Released under the MIT License.
          </a>
        </div>
        <div className="text-gray-400 text-sm">
          <span>Thank for </span>
          <a
            href="https://github.com/acha666/FuckJLC/tree/main"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-white hover:underline"
          >
            FuckJLC
          </a>{" "}
          <a
            href="https://github.com/HalfSweet/TransJLC"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-white hover:underline"
          >
            TransJLC
          </a>{" "}
          <a
            href="https://github.com/nextjlc/openjlc"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:text-white hover:underline"
          >
            OpenJLC
          </a>
        </div>
      </div>
    </footer>
  );
}

export default Footer;
