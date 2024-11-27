(function() {var type_impls = {
"ndarray":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/constructors.rs.html#116-219\">source</a><a href=\"#impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, D&gt; <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"docblock\"><p>Methods for read-write array views.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_shape\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/constructors.rs.html#145-151\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.from_shape\" class=\"fn\">from_shape</a>&lt;Sh&gt;(shape: Sh, xs: &amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.slice.html\">[A]</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"ndarray/struct.ShapeError.html\" title=\"struct ndarray::ShapeError\">ShapeError</a>&gt;<div class=\"where\">where\n    Sh: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"ndarray/struct.StrideShape.html\" title=\"struct ndarray::StrideShape\">StrideShape</a>&lt;D&gt;&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Create a read-write array view borrowing its data from a slice.</p>\n<p>Checks whether <code>dim</code> and <code>strides</code> are compatible with the slice’s\nlength, returning an <code>Err</code> if not compatible.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>ndarray::ArrayViewMut;\n<span class=\"kw\">use </span>ndarray::arr3;\n<span class=\"kw\">use </span>ndarray::ShapeBuilder;\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>s = [<span class=\"number\">0</span>, <span class=\"number\">1</span>, <span class=\"number\">2</span>, <span class=\"number\">3</span>, <span class=\"number\">4</span>, <span class=\"number\">5</span>, <span class=\"number\">6</span>, <span class=\"number\">7</span>, <span class=\"number\">8</span>, <span class=\"number\">9</span>, <span class=\"number\">10</span>, <span class=\"number\">11</span>, <span class=\"number\">12</span>];\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>a = ArrayViewMut::from_shape((<span class=\"number\">2</span>, <span class=\"number\">3</span>, <span class=\"number\">2</span>).strides((<span class=\"number\">1</span>, <span class=\"number\">4</span>, <span class=\"number\">2</span>)),\n                                     <span class=\"kw-2\">&amp;mut </span>s).unwrap();\n\na[[<span class=\"number\">0</span>, <span class=\"number\">0</span>, <span class=\"number\">0</span>]] = <span class=\"number\">1</span>;\n<span class=\"macro\">assert!</span>(\n    a == arr3(<span class=\"kw-2\">&amp;</span>[[[<span class=\"number\">1</span>, <span class=\"number\">2</span>],\n                 [<span class=\"number\">4</span>, <span class=\"number\">6</span>],\n                 [<span class=\"number\">8</span>, <span class=\"number\">10</span>]],\n                [[<span class=\"number\">1</span>, <span class=\"number\">3</span>],\n                 [<span class=\"number\">5</span>, <span class=\"number\">7</span>],\n                 [<span class=\"number\">9</span>, <span class=\"number\">11</span>]]])\n);\n<span class=\"macro\">assert!</span>(a.strides() == <span class=\"kw-2\">&amp;</span>[<span class=\"number\">1</span>, <span class=\"number\">4</span>, <span class=\"number\">2</span>]);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_shape_ptr\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/constructors.rs.html#204-209\">source</a><h4 class=\"code-header\">pub unsafe fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.from_shape_ptr\" class=\"fn\">from_shape_ptr</a>&lt;Sh&gt;(shape: Sh, ptr: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.pointer.html\">*mut A</a>) -&gt; Self<div class=\"where\">where\n    Sh: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"ndarray/struct.StrideShape.html\" title=\"struct ndarray::StrideShape\">StrideShape</a>&lt;D&gt;&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Create an <code>ArrayViewMut&lt;A, D&gt;</code> from shape information and a\nraw pointer to the elements.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>The caller is responsible for ensuring all of the following:</p>\n<ul>\n<li>\n<p>The elements seen by moving <code>ptr</code> according to the shape and strides\nmust live at least as long as <code>'a</code> and must not be aliased for the\nduration of <code>'a</code>.</p>\n</li>\n<li>\n<p><code>ptr</code> must be non-null and aligned, and it must be safe to\n<a href=\"https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset\"><code>.offset()</code></a> <code>ptr</code> by zero.</p>\n</li>\n<li>\n<p>It must be safe to <a href=\"https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset\"><code>.offset()</code></a> the pointer repeatedly along all\naxes and calculate the <code>count</code>s for the <code>.offset()</code> calls without\noverflow, even if the array is empty or the elements are zero-sized.</p>\n<p>In other words,</p>\n<ul>\n<li>\n<p>All possible pointers generated by moving along all axes must be in\nbounds or one byte past the end of a single allocation with element\ntype <code>A</code>. The only exceptions are if the array is empty or the element\ntype is zero-sized. In these cases, <code>ptr</code> may be dangling, but it must\nstill be safe to <a href=\"https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset\"><code>.offset()</code></a> the pointer along the axes.</p>\n</li>\n<li>\n<p>The offset in units of bytes between the least address and greatest\naddress by moving along all axes must not exceed <code>isize::MAX</code>. This\nconstraint prevents the computed offset, in bytes, from overflowing\n<code>isize</code> regardless of the starting point due to past offsets.</p>\n</li>\n<li>\n<p>The offset in units of <code>A</code> between the least address and greatest\naddress by moving along all axes must not exceed <code>isize::MAX</code>. This\nconstraint prevents overflow when calculating the <code>count</code> parameter to\n<a href=\"https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset\"><code>.offset()</code></a> regardless of the starting point due to past offsets.</p>\n</li>\n</ul>\n</li>\n<li>\n<p>The product of non-zero axis lengths must not exceed <code>isize::MAX</code>.</p>\n</li>\n<li>\n<p>Strides must be non-negative.</p>\n</li>\n</ul>\n<p>This function can use debug assertions to check some of these requirements,\nbut it’s not a complete check.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.reborrow\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/constructors.rs.html#213-218\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.reborrow\" class=\"fn\">reborrow</a>&lt;'b&gt;(self) -&gt; <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'b, A, D&gt;<div class=\"where\">where\n    'a: 'b,</div></h4></section></summary><div class=\"docblock\"><p>Convert the view into an <code>ArrayViewMut&lt;'b, A, D&gt;</code> where <code>'b</code> is a lifetime\noutlived by <code>'a'</code>.</p>\n</div></details></div></details>",0,"ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/conversions.rs.html#126-184\">source</a><a href=\"#impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, D&gt; <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"docblock\"><p>Methods for read-write array views.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_slice\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/conversions.rs.html#135-137\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.into_slice\" class=\"fn\">into_slice</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.slice.html\">[A]</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Return the array’s data as a slice, if it is contiguous and in standard order.\nReturn <code>None</code> otherwise.</p>\n<p>Note that while this is similar to <a href=\"ndarray/struct.ArrayBase.html#method.as_slice_mut\" title=\"method ndarray::ArrayBase::as_slice_mut\"><code>ArrayBase::as_slice_mut()</code></a>, this method transfers the\nview’s lifetime to the slice.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_slice_memory_order\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/conversions.rs.html#145-147\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.into_slice_memory_order\" class=\"fn\">into_slice_memory_order</a>(self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'a mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.slice.html\">[A]</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Return the array’s data as a slice, if it is contiguous.\nReturn <code>None</code> otherwise.</p>\n<p>Note that while this is similar to\n<a href=\"ndarray/struct.ArrayBase.html#method.as_slice_memory_order_mut\" title=\"method ndarray::ArrayBase::as_slice_memory_order_mut\"><code>ArrayBase::as_slice_memory_order_mut()</code></a>, this method transfers the\nview’s lifetime to the slice.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_cell_view\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/conversions.rs.html#155-162\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.into_cell_view\" class=\"fn\">into_cell_view</a>(self) -&gt; <a class=\"type\" href=\"ndarray/type.ArrayView.html\" title=\"type ndarray::ArrayView\">ArrayView</a>&lt;'a, <a class=\"struct\" href=\"ndarray/struct.MathCell.html\" title=\"struct ndarray::MathCell\">MathCell</a>&lt;A&gt;, D&gt;</h4></section></summary><div class=\"docblock\"><p>Return a shared view of the array with elements as if they were embedded in cells.</p>\n<p>The cell view itself can be copied and accessed without exclusivity.</p>\n<p>The view acts “as if” the elements are temporarily in cells, and elements\ncan be changed through shared references using the regular cell methods.</p>\n</div></details></div></details>",0,"ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/splitting.rs.html#131-169\">source</a><a href=\"#impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, D&gt; <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"docblock\"><p>Methods for read-write array views.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.split_at\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/splitting.rs.html#139-144\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.split_at\" class=\"fn\">split_at</a>(self, axis: <a class=\"struct\" href=\"ndarray/struct.Axis.html\" title=\"struct ndarray::Axis\">Axis</a>, index: <a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(Self, Self)</a></h4></section></summary><div class=\"docblock\"><p>Split the array view along <code>axis</code> and return one mutable view strictly\nbefore the split and one mutable view after the split.</p>\n<p><strong>Panics</strong> if <code>axis</code> or <code>index</code> is out of bounds.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.multi_slice_move\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/splitting.rs.html#163-168\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.multi_slice_move\" class=\"fn\">multi_slice_move</a>&lt;M&gt;(self, info: M) -&gt; M::<a class=\"associatedtype\" href=\"ndarray/trait.MultiSliceArg.html#associatedtype.Output\" title=\"type ndarray::MultiSliceArg::Output\">Output</a><div class=\"where\">where\n    M: <a class=\"trait\" href=\"ndarray/trait.MultiSliceArg.html\" title=\"trait ndarray::MultiSliceArg\">MultiSliceArg</a>&lt;'a, A, D&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Split the view into multiple disjoint slices.</p>\n<p>This is similar to <a href=\"ndarray/struct.ArrayBase.html#method.multi_slice_mut\" title=\"method ndarray::ArrayBase::multi_slice_mut\"><code>.multi_slice_mut()</code></a>, but <code>.multi_slice_move()</code>\nconsumes <code>self</code> and produces views with lifetimes matching that of\n<code>self</code>.</p>\n<p>See <a href=\"#slicing\"><em>Slicing</em></a> for full documentation. See also\n<a href=\"ndarray/trait.MultiSliceArg.html\" title=\"trait ndarray::MultiSliceArg\"><code>MultiSliceArg</code></a>, <a href=\"ndarray/prelude/macro.s.html\" title=\"macro ndarray::prelude::s\"><code>s!</code></a>, <a href=\"ndarray/trait.SliceArg.html\" title=\"trait ndarray::SliceArg\"><code>SliceArg</code></a>, and\n<a href=\"ndarray/struct.SliceInfo.html\" title=\"struct ndarray::SliceInfo\"><code>SliceInfo</code></a>.</p>\n<p><strong>Panics</strong> if any of the following occur:</p>\n<ul>\n<li>if any of the views would intersect (i.e. if any element would appear in multiple slices)</li>\n<li>if an index is out of bounds or step size is zero</li>\n<li>if <code>D</code> is <code>IxDyn</code> and <code>info</code> does not match the number of array axes</li>\n</ul>\n</div></details></div></details>",0,"ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+Dim%3C%5Busize;+0%5D%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/conversions.rs.html#104-123\">source</a><a href=\"#impl-ArrayBase%3CViewRepr%3C%26mut+A%3E,+Dim%3C%5Busize;+0%5D%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A&gt; <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, <a class=\"type\" href=\"ndarray/type.Ix0.html\" title=\"type ndarray::Ix0\">Ix0</a>&gt;</h3></section></summary><div class=\"docblock\"><p>Methods specific to <code>ArrayViewMut0</code>.</p>\n<p><em><strong>See also all methods for <a href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\"><code>ArrayViewMut</code></a> and <a href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\"><code>ArrayBase</code></a></strong></em></p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_scalar\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/conversions.rs.html#120-122\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.into_scalar\" class=\"fn\">into_scalar</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a></h4></section></summary><div class=\"docblock\"><p>Consume the mutable view and return a mutable reference to the single element in the array.</p>\n<p>The lifetime of the returned reference matches the lifetime of the data\nthe array view was pointing to.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>ndarray::{arr0, Array0};\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>array: Array0&lt;f64&gt; = arr0(<span class=\"number\">5.</span>);\n<span class=\"kw\">let </span>view = array.view_mut();\n<span class=\"kw\">let </span>scalar = view.into_scalar();\n<span class=\"kw-2\">*</span>scalar = <span class=\"number\">7.</span>;\n<span class=\"macro\">assert_eq!</span>(scalar, <span class=\"kw-2\">&amp;</span><span class=\"number\">7.</span>);\n<span class=\"macro\">assert_eq!</span>(array[()], <span class=\"number\">7.</span>);</code></pre></div>\n</div></details></div></details>",0,"ndarray::aliases::ArrayViewMut0"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ArrayBase%3CViewRepr%3C%26mut+Complex%3CT%3E%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/splitting.rs.html#171-207\">source</a><a href=\"#impl-ArrayBase%3CViewRepr%3C%26mut+Complex%3CT%3E%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T, D&gt; <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;, D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.split_complex\" class=\"method\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/splitting.rs.html#198-206\">source</a><h4 class=\"code-header\">pub fn <a href=\"ndarray/type.ArrayViewMut.html#tymethod.split_complex\" class=\"fn\">split_complex</a>(self) -&gt; <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;<a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, T, D&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Splits the view into views of the real and imaginary components of the\nelements.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>ndarray::prelude::<span class=\"kw-2\">*</span>;\n<span class=\"kw\">use </span>num_complex::{Complex, Complex64};\n\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>arr = <span class=\"macro\">array!</span>[\n    [Complex64::new(<span class=\"number\">1.</span>, <span class=\"number\">2.</span>), Complex64::new(<span class=\"number\">3.</span>, <span class=\"number\">4.</span>)],\n    [Complex64::new(<span class=\"number\">5.</span>, <span class=\"number\">6.</span>), Complex64::new(<span class=\"number\">7.</span>, <span class=\"number\">8.</span>)],\n    [Complex64::new(<span class=\"number\">9.</span>, <span class=\"number\">10.</span>), Complex64::new(<span class=\"number\">11.</span>, <span class=\"number\">12.</span>)],\n];\n\n<span class=\"kw\">let </span>Complex { <span class=\"kw-2\">mut </span>re, <span class=\"kw-2\">mut </span>im } = arr.view_mut().split_complex();\n<span class=\"macro\">assert_eq!</span>(re, <span class=\"macro\">array!</span>[[<span class=\"number\">1.</span>, <span class=\"number\">3.</span>], [<span class=\"number\">5.</span>, <span class=\"number\">7.</span>], [<span class=\"number\">9.</span>, <span class=\"number\">11.</span>]]);\n<span class=\"macro\">assert_eq!</span>(im, <span class=\"macro\">array!</span>[[<span class=\"number\">2.</span>, <span class=\"number\">4.</span>], [<span class=\"number\">6.</span>, <span class=\"number\">8.</span>], [<span class=\"number\">10.</span>, <span class=\"number\">12.</span>]]);\n\nre[[<span class=\"number\">0</span>, <span class=\"number\">1</span>]] = <span class=\"number\">13.</span>;\nim[[<span class=\"number\">2</span>, <span class=\"number\">0</span>]] = <span class=\"number\">14.</span>;\n\n<span class=\"macro\">assert_eq!</span>(arr[[<span class=\"number\">0</span>, <span class=\"number\">1</span>]], Complex64::new(<span class=\"number\">13.</span>, <span class=\"number\">4.</span>));\n<span class=\"macro\">assert_eq!</span>(arr[[<span class=\"number\">2</span>, <span class=\"number\">0</span>]], Complex64::new(<span class=\"number\">9.</span>, <span class=\"number\">14.</span>));</code></pre></div>\n</div></details></div></details>",0,"ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26mut+ArrayBase%3CS,+D%3E%3E-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/arraytraits.rs.html#359-368\">source</a><a href=\"#impl-From%3C%26mut+ArrayBase%3CS,+D%3E%3E-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, S, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a mut <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;&gt; for <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"ndarray/trait.DataMut.html\" title=\"trait ndarray::DataMut\">DataMut</a>&lt;Elem = A&gt;,\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"docblock\"><p>Implementation of <code>ArrayViewMut::from(&amp;mut A)</code> where <code>A</code> is an array.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/arraytraits.rs.html#365-367\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(array: &amp;'a mut <a class=\"struct\" href=\"ndarray/struct.ArrayBase.html\" title=\"struct ndarray::ArrayBase\">ArrayBase</a>&lt;S, D&gt;) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a read-write array view of the array.</p>\n</div></details></div></details>","From<&'a mut ArrayBase<S, D>>","ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26mut+Slice%3E-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+Dim%3C%5Busize;+1%5D%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/arraytraits.rs.html#339-356\">source</a><a href=\"#impl-From%3C%26mut+Slice%3E-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+Dim%3C%5Busize;+1%5D%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, Slice&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut Slice</a>&gt; for <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, <a class=\"type\" href=\"ndarray/type.Ix1.html\" title=\"type ndarray::Ix1\">Ix1</a>&gt;<div class=\"where\">where\n    Slice: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.slice.html\">[A]</a>&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section></summary><div class=\"docblock\"><p>Implementation of <code>ArrayViewMut::from(&amp;mut S)</code> where <code>S</code> is a slice or sliceable.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/arraytraits.rs.html#346-355\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(slice: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut Slice</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a one-dimensional read-write array view of the data in <code>slice</code>.</p>\n<p><strong>Panics</strong> if the slice length is greater than <code>isize::MAX</code>.</p>\n</div></details></div></details>","From<&'a mut Slice>","ndarray::aliases::ArrayViewMut1"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IndexLonger%3CI%3E-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/indexing.rs.html#145-207\">source</a><a href=\"#impl-IndexLonger%3CI%3E-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, I, A, D&gt; <a class=\"trait\" href=\"ndarray/trait.IndexLonger.html\" title=\"trait ndarray::IndexLonger\">IndexLonger</a>&lt;I&gt; for <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"ndarray/trait.NdIndex.html\" title=\"trait ndarray::NdIndex\">NdIndex</a>&lt;D&gt;,\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/indexing.rs.html#164-172\">source</a><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"ndarray/trait.IndexLonger.html#tymethod.index\" class=\"fn\">index</a>(self, index: I) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a></h4></section></summary><div class=\"docblock\"><p>Convert a mutable array view to a mutable reference of a element.</p>\n<p>This method is like <code>IndexMut::index_mut</code> but with a longer lifetime\n(matching the array view); which we can only do for the array view and\nnot in the <code>Index</code> trait.</p>\n<p>See also <a href=\"ndarray/struct.ArrayBase.html#method.get_mut\" title=\"method ndarray::ArrayBase::get_mut\">the <code>get_mut</code> method</a> which works for all arrays and array\nviews.</p>\n<p><strong>Panics</strong> if index is out of bounds.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/indexing.rs.html#182-190\">source</a><a href=\"#method.get\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"ndarray/trait.IndexLonger.html#tymethod.get\" class=\"fn\">get</a>(self, index: I) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Convert a mutable array view to a mutable reference of a element, with\nchecked access.</p>\n<p>See also <a href=\"ndarray/struct.ArrayBase.html#method.get_mut\" title=\"method ndarray::ArrayBase::get_mut\">the <code>get_mut</code> method</a> which works for all arrays and array\nviews.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.uget\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/impl_views/indexing.rs.html#201-206\">source</a><a href=\"#method.uget\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"ndarray/trait.IndexLonger.html#tymethod.uget\" class=\"fn\">uget</a>(self, index: I) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a></h4></section></summary><div class=\"docblock\"><p>Convert a mutable array view to a mutable reference of a element without\nboundary check.</p>\n<p>See also <a href=\"ndarray/struct.ArrayBase.html#method.uget_mut\" title=\"method ndarray::ArrayBase::uget_mut\">the <code>uget_mut</code> method</a> which works for all arrays and array\nviews.</p>\n<p><strong>Note:</strong> only unchecked for non-debug builds of ndarray.</p>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"ndarray/trait.IndexLonger.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a></h4></section></summary><div class='docblock'>The type of the reference to the element that is produced, including\nits lifetime.</div></details></div></details>","IndexLonger<I>","ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntoIterator-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/arraytraits.rs.html#243-253\">source</a><a href=\"#impl-IntoIterator-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a> for <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;<div class=\"where\">where\n    D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a></h4></section></summary><div class='docblock'>The type of the elements being iterated over.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.IntoIter\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.IntoIter\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter\" class=\"associatedtype\">IntoIter</a> = <a class=\"struct\" href=\"ndarray/iter/struct.IterMut.html\" title=\"struct ndarray::iter::IterMut\">IterMut</a>&lt;'a, A, D&gt;</h4></section></summary><div class='docblock'>Which kind of iterator are we turning this into?</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_iter\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/arraytraits.rs.html#250-252\">source</a><a href=\"#method.into_iter\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter\" class=\"fn\">into_iter</a>(self) -&gt; Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter\" title=\"type core::iter::traits::collect::IntoIterator::IntoIter\">IntoIter</a></h4></section></summary><div class='docblock'>Creates an iterator from a value. <a href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter\">Read more</a></div></details></div></details>","IntoIterator","ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-NdProducer-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ndarray/zip/ndproducer.rs.html#237-281\">source</a><a href=\"#impl-NdProducer-for-ArrayBase%3CViewRepr%3C%26mut+A%3E,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, A, D: <a class=\"trait\" href=\"ndarray/trait.Dimension.html\" title=\"trait ndarray::Dimension\">Dimension</a>&gt; <a class=\"trait\" href=\"ndarray/trait.NdProducer.html\" title=\"trait ndarray::NdProducer\">NdProducer</a> for <a class=\"type\" href=\"ndarray/type.ArrayViewMut.html\" title=\"type ndarray::ArrayViewMut\">ArrayViewMut</a>&lt;'a, A, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"ndarray/trait.NdProducer.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;'a mut A</a></h4></section></summary><div class='docblock'>The element produced per iteration.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Dim\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Dim\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"ndarray/trait.NdProducer.html#associatedtype.Dim\" class=\"associatedtype\">Dim</a> = D</h4></section></summary><div class='docblock'>Dimension type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.raw_dim\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ndarray/zip/ndproducer.rs.html#245-247\">source</a><a href=\"#method.raw_dim\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"ndarray/trait.NdProducer.html#tymethod.raw_dim\" class=\"fn\">raw_dim</a>(&amp;self) -&gt; Self::<a class=\"associatedtype\" href=\"ndarray/trait.NdProducer.html#associatedtype.Dim\" title=\"type ndarray::NdProducer::Dim\">Dim</a></h4></section></summary><div class='docblock'>Return the shape of the producer.</div></details></div></details>","NdProducer","ndarray::aliases::ArrayViewMut0","ndarray::aliases::ArrayViewMut1","ndarray::aliases::ArrayViewMut2","ndarray::aliases::ArrayViewMut3","ndarray::aliases::ArrayViewMut4","ndarray::aliases::ArrayViewMut5","ndarray::aliases::ArrayViewMut6","ndarray::aliases::ArrayViewMutD"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()