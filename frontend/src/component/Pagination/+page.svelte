<script lang="ts">
  import { error } from "@sveltejs/kit";

  export let pageIndex: number;
  export let totalPageCount: number; // must be greater than 1
  export let getPageLink: (index: number) => string;

  let showHeadDots = false;
  let showTailDots = false;
  let showPrevArrow = false;
  let showNextArrow = false;
  let innerPagesToShow: number[] = [];

  if(totalPageCount < 1) {
    throw error(400, { message: "Invalid page count." });
  }

  // * ページ数が5ページ以内のとき
  //   * すべてのページ番号を表示する
  // * それ以外のとき
  //   * 現在のページとその前後の1ページを表示。
  //   * 先頭ページと末尾ページは必ず表示する。
  //   * 現在のページと先頭ページの差が 2 以上の場合は間に ... を表示する。
  //   * 現在のページと末尾ページの差が 2 以上の場合は間に ... を表示する。

  $: {
    if(pageIndex >= totalPageCount) {
      pageIndex = totalPageCount - 1;
    }

    innerPagesToShow = [];

    if(totalPageCount <= 5) {
      const tmp = [...Array(totalPageCount).keys()];
      tmp.pop();
      tmp.shift();
      innerPagesToShow = tmp;
    } else {

      const tmp =
      (pageIndex === 0 ? [pageIndex, pageIndex + 1, pageIndex + 2] :
      pageIndex === totalPageCount - 1 ? [pageIndex - 2, pageIndex - 1, pageIndex] :
      [pageIndex - 1, pageIndex, pageIndex + 1]
      );

      for(const i of tmp) {
        if(i < 1 || i >= (totalPageCount-1)) {
          continue;
        }
        innerPagesToShow.push(i);
      }
    }

    showHeadDots = totalPageCount >= 6 && pageIndex >= 3;
    showTailDots = totalPageCount >= 6 && pageIndex <= (totalPageCount-4);
    showPrevArrow = pageIndex !== 0;
    showNextArrow = pageIndex !== totalPageCount-1;
    innerPagesToShow = [...innerPagesToShow];
  }
</script>

<div class="layout-pagination">
  <div class="page-link-list">
    <a class="page-link" class:hide={showPrevArrow === false} href={getPageLink(pageIndex-1)}>{"<<"}</a>
    <a class="page-link" class:current={pageIndex === 0} href={getPageLink(0)}>1</a>
    {#if showHeadDots}
    <div class="page-link page-link-dots">…</div>
    {/if}
    {#each innerPagesToShow as i}
    <a class="page-link" class:current={pageIndex === i} href={getPageLink(i)}>{i+1}</a>
    {/each}
    {#if showTailDots}
    <div class="page-link page-link-dots">…</div>
    {/if}
    <a class="page-link" class:current={pageIndex === totalPageCount - 1} href={getPageLink(totalPageCount-1)}>{totalPageCount}</a>
    <a class="page-link page-link-arrow" class:hide={showNextArrow === false} href={getPageLink(pageIndex+1)}>{">>"}</a>
  </div>
</div>

<style>
  .page-link-list {
    display: flex;
    align-items: center;
    gap: 32px;
  }

  .page-link {
    color: #888;
    font-size: 18px;
    font-style: normal;
    font-weight: 700;
    line-height: normal;
    letter-spacing: 1.5px;
    display: block;
    text-decoration: none;

    &.current {
      color: #3387FF;
      pointer-events: none;
    }

    &.hide {
      visibility: hidden;
      pointer-events: none;
    }

    &.page-link-dots {
      margin: 0px -15px;
      transform: translate(0px, 7px);
    }

    &.page-link-arrow {
      margin: 0px 10px;
    }
  }
</style>